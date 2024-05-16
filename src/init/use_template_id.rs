use std::env::current_dir;

use actions_templates::{config::Config, ActionConfig};
use serde_json::Value;

use crate::{error, info, path_configs::WritePath, success, CARGO_ACTIONS_URL};

pub fn use_template_id(id: &str) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let client = reqwest::Client::new();

            let res = client
                .get(format!("{CARGO_ACTIONS_URL}/api/template/{id}"))
                .send()
                .await?;

            if res.status().is_success() {
                info!("upload template success");
                let mut data = res.json::<Value>().await?;
                if data.is_null() {
                    error!("template {id} not found");
                } else {
                    let config_string = &data["config"];
                    let new_config: Config = serde_json::from_str(config_string.as_str().unwrap())?;
                    data["config"] = serde_json::to_value(new_config)?;
                    let actions_template = serde_json::from_value::<ActionConfig>(data)?;
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
                }
            } else {
                let error = res.text().await?;
                error!("get template error: {error}");
            }

            Ok::<(), anyhow::Error>(())
        })?;
    Ok(())
}
