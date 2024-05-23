use anyhow::{anyhow, bail};

use cynic::{http::ReqwestExt, MutationBuilder, QueryBuilder};

use crate::{
    client::author_client,
    error,
    graphql::{FavoriteTemplates, IncreaseTemplate, IncreaseTemplateVariables, UserTemplates},
    path_configs::{PathConfigs, SelectPathConfig},
    CARGO_ACTIONS_URL,
};

pub fn run() -> anyhow::Result<()> {
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

async fn get_user_templates() -> anyhow::Result<PathConfigs> {
    let query = FavoriteTemplates::build(());
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

    if let Some(FavoriteTemplates {
        favorite_templates: UserTemplates { templates },
    }) = res.data
    {
        if templates.is_empty() {
            bail!("您没有收藏任何模版")
        }
        Ok(templates.into())
    } else {
        Err(anyhow!("获取模板失败"))
    }
}
