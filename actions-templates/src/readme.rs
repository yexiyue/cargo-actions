use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReadME(String);

impl From<String> for ReadME {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl ReadME {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = std::fs::read_to_string(path)?;
        Ok(Self(file))
    }
}

impl Deref for ReadME {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
