use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template(pub String);

impl From<String> for Template {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Template {
    pub fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = std::fs::read_to_string(path)?;

        Ok(Self(file))
    }

    pub fn render_to_string<D: Serialize>(&self, data: &D) -> Result<String> {
        let handlebars = handlebars::Handlebars::new();
        Ok(handlebars.render_template(self, data)?)
    }
}

impl Deref for Template {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
