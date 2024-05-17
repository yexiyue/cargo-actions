use std::fmt::Display;

use super::CommonFields;
#[cfg(feature = "dialoguer")]
use super::DialoguerValue;
#[cfg(feature = "dialoguer")]
use dialoguer::theme::ColorfulTheme;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Select {
    #[serde(flatten)]
    pub common: CommonFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<usize>,
    pub options: Vec<ItemOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemOption {
    pub label: String,
    pub value: Value,
}

#[cfg(feature = "dialoguer")]
impl Display for ItemOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label)
    }
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for Select {
    type Value = Value;
    fn dialoguer_value(&self) -> anyhow::Result<Self::Value> {
        let selected = if self.default.is_none() {
            dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.common.prompt)
                .items(&self.options)
                .interact()?
        } else {
            dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.common.prompt)
                .items(&self.options)
                .default(self.default.unwrap())
                .interact()?
        };
        Ok(self.options[selected].value.clone())
    }
}
