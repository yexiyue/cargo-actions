use actions_templates::ActionConfig;
use axum::http::HeaderValue;
use clap::Args;
use dialogue_macro::Asker;
use serde_json::{json, Value};

use crate::{error, info, token::Token, Run, CARGO_ACTIONS_FRONT_URL, CARGO_ACTIONS_URL};

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
        let token = Token::read()?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async move {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.append(
                    reqwest::header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.token))?,
                );

                let client = reqwest::Client::builder()
                    .default_headers(headers)
                    .build()?;

                let res = client
                    .post(format!("{CARGO_ACTIONS_URL}/api/template"))
                    .json(&json!({
                        "name":action_config.config.name,
                        "config":action_config.config,
                        "readme":action_config.readme,
                        "template":action_config.template,
                        "user_id":token.user.id,
                    }))
                    .send()
                    .await?;

                if res.status().is_success() {
                    info!("upload template success");
                    let data = res.json::<Value>().await?;
                    let id = &data["id"];
                    info!("template url: {CARGO_ACTIONS_FRONT_URL}/{id}");
                } else {
                    let error = res.text().await?;
                    error!("upload template error: {error}");
                }

                Ok::<(), anyhow::Error>(())
            })?;
        Ok(())
    }
}
