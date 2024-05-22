use crate::{
    client::{author_client, get_user_id},
    error,
    graphql::{
        AddTemplateTags, AddTemplateTagsVariables, Category, CategoryAndTags, Json, Tag,
        TemplateCreateInput, TemplateTagInput, UploadTemplate, UploadTemplateVariables,
    },
    info, Run, CARGO_ACTIONS_FRONT_URL, CARGO_ACTIONS_URL,
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
    #[multiselect(prompt = "请选择该模版的标签")]
    pub tags_id: Vec<Tag>,
    #[confirm(prompt = "是否公开该模版？", default = true)]
    pub is_public: bool,
}

#[derive(Debug, Args, Asker)]
pub struct UploadArgs {
    /// The template path to upload
    #[input(prompt = "请输入actions template文件夹路径：")]
    path: Option<String>,
}

impl Run for UploadArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        if self.path.is_none() {
            *self = UploadArgs::asker().path().finish();
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
                    let inputs = Inputs::asker()
                        .category_id(&res.categories, 0)
                        .tags_id(&res.tags)
                        .is_public()
                        .finish();

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

                    if let Some(data) = res.data {
                        let add_tags_mutation = AddTemplateTags::build(AddTemplateTagsVariables {
                            input: TemplateTagInput {
                                tag_id: inputs.tags_id.iter().map(|x| x.id).collect(),
                                template_id: data.create_template,
                            },
                        });
                        client
                            .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                            .run_graphql(add_tags_mutation)
                            .await?;

                        info!("upload template success: {}", data.create_template);
                    }

                    if let Some(errors) = res.errors {
                        for error in errors {
                            if error.message == "Unauthorized" {
                                error!("{} 请重新登陆", error.message);
                            } else {
                                error!("{}", error.message);
                            }
                        }
                    }
                } else {
                    if let Some(errors) = category_tags_res.errors {
                        for error in errors {
                            error!("{}", error.message);
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            })?;
        Ok(())
    }
}
