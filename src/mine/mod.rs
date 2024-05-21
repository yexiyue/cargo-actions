use anyhow::{anyhow, bail};
use clap::Args;
use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};

use crate::{
    client::author_client,
    error,
    graphql::{IncreaseTemplate, IncreaseTemplateVariables, UserTemplates},
    path_configs::{PathConfigs, SelectPathConfig},
    Run, CARGO_ACTIONS_URL,
};

#[derive(Debug, Args)]
pub struct MineArgs {
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    pub favorite: bool,
}

impl Run for MineArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                let templates = get_user_templates().await?;
                let select_config = SelectPathConfig::asker()
                    .action_config(&templates, 0)
                    .finish();
                if let Some(id) = select_config.action_config.id {
                    let (client, _) = author_client()?;
                    let mutation = IncreaseTemplate::build(IncreaseTemplateVariables { id });
                    client
                        .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
                        .run_graphql(mutation)
                        .await?;
                }
                select_config.write_template()?;
                Ok::<(), anyhow::Error>(())
            })
    }
}

async fn get_user_templates() -> anyhow::Result<PathConfigs> {
    let query = UserTemplates::build(());
    let (client, _) = author_client()?;
    let res = client
        .post(format!("{CARGO_ACTIONS_URL}/api/graphql"))
        .run_graphql(query)
        .await?;
    if let Some(errors) = res.errors {
        for error in errors {
            if error.message == "Unauthorized" {
                bail!("{} 请重新登陆", error.message);
            } else {
                error!("{}", error.message);
            }
        }
    }

    if let Some(data) = res.data {
        Ok(data.templates_by_user.into())
    } else {
        Err(anyhow!("获取模板失败"))
    }
}
