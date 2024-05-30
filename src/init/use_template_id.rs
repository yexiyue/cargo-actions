use std::env::current_dir;

use actions_templates::{config::Config, template::Template, ActionConfig};
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};
use serde_json::Value;

use crate::{
    error,
    graphql::{IncreaseTemplate, IncreaseTemplateVariables, QueryTemplate, QueryTemplateVariables},
    info,
    path_configs::WritePath,
    success, CARGO_ACTIONS_URL,
};

pub fn use_template_id(id: &str) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let client = reqwest::Client::new();
            let query = QueryTemplate::build(QueryTemplateVariables {
                id: id.parse::<i32>()?,
            });

            let res = client
                .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                .run_graphql(query)
                .await?;

            if let Some(QueryTemplate { template_by_id }) = res.data {
                success!("download template success");

                if let Some(data) = template_by_id {
                    let config_string = &data.config;

                    let new_config: Value = serde_json::from_str(config_string.as_str())?;
                    let new_config: Config = serde_json::from_str(new_config.as_str().unwrap())?;

                    let actions_template = ActionConfig {
                        id: Some(data.id),
                        config: new_config,
                        template: Template(data.template),
                        readme: None,
                    };

                    let default_write_path = current_dir()?
                        .join(".github/workflows")
                        .join(format!("{}.yaml", actions_template.config.name))
                        .to_string_lossy()
                        .to_string();
                    let write_path = WritePath::input_and_confirm(default_write_path)?;
                    actions_template.write_template(write_path)?;

                    success!("write success");

                    if actions_template.config.success_message.is_some() {
                        info!("{}", actions_template.config.success_message.unwrap());
                    }
                } else {
                    error!("template {id} not found");
                }
            }

            if let Some(errors) = res.errors {
                for error in errors {
                    error!("{}", error.message);
                }
            }

            let mutation = IncreaseTemplate::build(IncreaseTemplateVariables {
                id: id.parse::<i32>().unwrap(),
            });

            client
                .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                .run_graphql(mutation)
                .await?;
            Ok::<(), anyhow::Error>(())
        })?;
    Ok(())
}
