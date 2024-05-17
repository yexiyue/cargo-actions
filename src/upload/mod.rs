use crate::{
    client::author_client,
    error,
    graphql::{Json, TemplateCreateInput, UploadTemplate, UploadTemplateVariables},
    info, Run, CARGO_ACTIONS_FRONT_URL, CARGO_ACTIONS_URL,
};
use actions_templates::ActionConfig;
use clap::Args;
use cynic::{http::ReqwestExt, MutationBuilder};
use dialogue_macro::Asker;

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
        let action_config = ActionConfig::from_dir(path)?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async move {
                let (client, token) = author_client()?;
                let mutation = UploadTemplate::build(UploadTemplateVariables {
                    input: TemplateCreateInput {
                        name: &action_config.config.name,
                        config: Json(serde_json::to_string(&action_config.config)?),
                        readme: action_config.readme.as_ref().map(|readme| readme.as_str()),
                        template: action_config.template.as_str(),
                        user_id: token.user.id as i32,
                        category_id: None,
                        source_code_url: None,
                    },
                });

                let res = client
                    .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                    .run_graphql(mutation)
                    .await?;

                if let Some(data) = res.data {
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
                Ok::<(), anyhow::Error>(())
            })?;
        Ok(())
    }
}
