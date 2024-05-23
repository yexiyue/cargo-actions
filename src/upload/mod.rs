use crate::{
    client::{author_client, get_user_id},
    error,
    graphql::{
        AddTemplateTags, AddTemplateTagsVariables, Category, CategoryAndTags, Json, Tag,
        TemplateCreateInput, TemplateTagInput, TemplateTags, TemplateTagsVariables,
        TemplateUpdateInput, UpdateTemplate, UpdateTemplateVariables, UploadTemplate,
        UploadTemplateVariables,
    },
    info, success, Run, CARGO_ACTIONS_FRONT_URL, CARGO_ACTIONS_URL,
};
use actions_templates::ActionConfig;
use anyhow::anyhow;
use clap::Args;
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use dialogue_macro::Asker;
use std::{env, path::Path};

#[derive(Debug, Asker)]
pub struct Inputs {
    #[select(prompt = "请选择该模版的分类", with_default = true)]
    pub category_id: Category,
    #[multiselect(prompt = "请选择该模版的标签", with_default = true)]
    pub tags_id: Vec<Tag>,
    #[confirm(prompt = "是否公开该模版？", default = true, with_default = true)]
    pub is_public: bool,
}

#[derive(Debug, Args, Asker)]
pub struct UploadArgs {
    /// Specifies the local path to the actions template folder to be uploaded.
    #[input(prompt = "请输入actions template文件夹路径：")]
    path: Option<String>,

    /// Specifies whether to update the existing template.
    #[confirm(prompt = "您想要更新您的模版吗？", default = false)]
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    update: Option<bool>,

    /// Specifies the ID of the template to be updated.
    #[input(prompt = "请输入actions template模版ID：")]
    #[arg(short, long)]
    id: Option<i32>,
}

impl Run for UploadArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        println!("self:{self:?}");
        if !self.update.unwrap() {
            let args = UploadArgs::asker().update().finish();
            self.update = args.update;
        }

        if self.update.unwrap() && self.id.is_none() {
            let args = UploadArgs::asker().id().finish();
            self.id = args.id;
        }

        if self.path.is_none() {
            let args = UploadArgs::asker().path().finish();
            self.path = args.path;
        }

        let path = self.path.as_ref().unwrap();
        let mut path = Path::new(path).to_path_buf();

        if path.is_relative() {
            let current_dir = env::current_dir()?;
            path = current_dir.join(path);
        }
        if path.is_file() {
            return Err(anyhow!("actions template文件夹路径不能是文件"));
        }

        let action_config = ActionConfig::from_dir(path)?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async move {
                let user_id = get_user_id().await?;
                let (client, _) = author_client()?;
                let category_tags_query = CategoryAndTags::build(());

                let category_tags_res = client
                    .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                    .run_graphql(category_tags_query)
                    .await?;
                if let Some(res) = category_tags_res.data {
                    let inputs = if self.update.unwrap() {
                        let TemplateTags {
                            template_tags,
                            template_by_id,
                        } = get_template_tags(self.id.unwrap()).await?;
                        let position = res
                            .categories
                            .categories
                            .iter()
                            .position(|i| i.id == template_by_id.category_id);

                        let tag_positions = res
                            .tags
                            .tags
                            .iter()
                            .map(|i| template_tags.contains(i))
                            .collect::<Vec<_>>();

                        Inputs::asker()
                            .category_id(&res.categories.categories, position.unwrap())
                            .tags_id(&res.tags.tags, &tag_positions)
                            .is_public(template_by_id.is_public)
                            .finish()
                    } else {
                        Inputs::asker()
                            .category_id(&res.categories.categories, 0)
                            .tags_id(&res.tags.tags, &[])
                            .is_public(true)
                            .finish()
                    };

                    let template_id = if !self.update.unwrap() {
                        let mutation = UploadTemplate::build(UploadTemplateVariables {
                            input: TemplateCreateInput {
                                name: &action_config.config.name,
                                config: Json(serde_json::to_string(&action_config.config)?),
                                readme: action_config.readme.as_ref().map(|readme| readme.as_str()),
                                template: action_config.template.as_str(),
                                user_id,
                                category_id: inputs.category_id.id,
                                source_code_url: None,
                                is_public: inputs.is_public,
                            },
                        });
                        let res = client
                            .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                            .run_graphql(mutation)
                            .await?;
                        if let Some(errors) = res.errors {
                            for error in errors {
                                error!("{}", error.message);
                            }
                        }

                        res.data.map(|data| data.create_template)
                    } else {
                        let mutation = UpdateTemplate::build(UpdateTemplateVariables {
                            input: TemplateUpdateInput {
                                name: Some(&action_config.config.name),
                                config: Some(Json(serde_json::to_string(&action_config.config)?)),
                                readme: action_config.readme.as_ref().map(|readme| readme.as_str()),
                                template: Some(action_config.template.as_str()),
                                category_id: Some(inputs.category_id.id),
                                source_code_url: None,
                                is_public: Some(inputs.is_public),
                            },
                            id: self.id.unwrap(),
                        });
                        let res = client
                            .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                            .run_graphql(mutation)
                            .await?;

                        if let Some(errors) = res.errors {
                            for error in errors {
                                error!("{}", error.message);
                            }
                        }

                        res.data.map(|data| data.update_template.id)
                    };

                    if let Some(template_id) = template_id {
                        let add_tags_mutation = AddTemplateTags::build(AddTemplateTagsVariables {
                            input: TemplateTagInput {
                                tag_id: inputs.tags_id.iter().map(|x| x.id).collect(),
                                template_id,
                            },
                        });
                        client
                            .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                            .run_graphql(add_tags_mutation)
                            .await?;

                        if self.update.unwrap() {
                            success!("模版更新成功");
                        } else {
                            success!("模版上传成功");
                        }
                        info!(
                            "\n template url: {CARGO_ACTIONS_FRONT_URL}/template/{}",
                            template_id
                        );
                    }
                } else if let Some(errors) = category_tags_res.errors {
                    for error in errors {
                        error!("{}", error.message);
                    }
                }

                Ok::<(), anyhow::Error>(())
            })?;
        Ok(())
    }
}

async fn get_template_tags(id: i32) -> anyhow::Result<TemplateTags> {
    let (client, _) = author_client()?;
    let tags_query = TemplateTags::build(TemplateTagsVariables { id });

    let tags_res = client
        .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
        .run_graphql(tags_query)
        .await?;
    if let Some(data) = tags_res.data {
        return Ok(data);
    }
    if let Some(errors) = tags_res.errors {
        for error in errors {
            error!("{}", error.message);
        }
    }
    Err(anyhow!("获取模版：{id}失败"))
}
