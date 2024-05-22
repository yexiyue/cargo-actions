use crate::{error, CARGO_ACTIONS_URL};
use crate::{graphql::GetUserId, token::Token};
use anyhow::anyhow;
use cynic::http::ReqwestExt;
use cynic::QueryBuilder;
use reqwest::header::HeaderValue;
use serde_json::{json, Value};

pub fn author_client() -> anyhow::Result<(reqwest::Client, Token)> {
    let token = Token::read()?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token.token))?,
    );
    Ok((
        reqwest::Client::builder()
            .default_headers(headers)
            .build()?,
        token,
    ))
}

pub async fn get_user_id() -> anyhow::Result<i32> {
    let (client, mut token) = author_client()?;
    let query_id = GetUserId::build(());
    let res = client
        .post(format!("{}/api/graphql", CARGO_ACTIONS_URL))
        .run_graphql(query_id)
        .await?;

    if let Some(errors) = res.errors {
        for error in errors {
            if error.message == "token expired" {
                let res = client
                    .post(format!("{}/api/refresh", CARGO_ACTIONS_URL))
                    .json(&json!({
                        "id":token.user.id,
                        "refresh_token":token.refresh_token
                    }))
                    .send()
                    .await;

                match res {
                    Ok(data) => {
                        let value: Value = data.json().await?;
                        if let Some(code) = value["code"].as_number() {
                            if code.as_u64() == Some(500) {
                                return Err(anyhow!("刷新token失败! 请重新登陆"));
                            }
                        }
                        token.token = value["token"].as_str().unwrap().to_string();
                        token.access_token = value["access_token"].as_str().unwrap().to_string();
                        token.refresh_token = value["refresh_token"].as_str().unwrap().to_string();
                        token.save()?;
                        return Ok(token.user.id);
                    }
                    Err(_) => return Err(anyhow!("刷新token失败! 请重新登陆")),
                };
            } else {
                error!("{}", error.message);
            }
        }
    }

    if let Some(data) = res.data {
        Ok(data.user.id)
    } else {
        Err(anyhow!("获取用户id失败"))
    }
}
