use reqwest::header::HeaderValue;

use crate::token::Token;

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
