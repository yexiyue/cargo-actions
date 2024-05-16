use crate::prompts::Prompts;
use serde::{Deserialize, Serialize};
use std::path::Path;

use anyhow::Result;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 简短的介绍，用于cli
    pub description: Option<String>,
    /// 如果不传默认使用 name.yaml.hbs模版
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub prompts: Prompts,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_message: Option<String>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }
}
