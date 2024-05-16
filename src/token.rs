use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub access_token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub create_at: Option<String>,
    pub id: u64,
    pub username: String,
}

impl Token {
    pub fn read() -> anyhow::Result<Self> {
        let dir = env!("HOME");
        let file_path = Path::new(dir).join(".cargo-actions/token.json");
        if file_path.exists() {
            let file = File::open(file_path)?;
            let token: Token = serde_json::from_reader(file)?;
            return Ok(token);
        } else {
            return Err(anyhow::anyhow!("please login first"));
        }
    }
}
