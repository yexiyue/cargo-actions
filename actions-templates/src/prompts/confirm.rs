use super::CommonFields;
#[cfg(feature = "dialoguer")]
use super::DialoguerValue;
#[cfg(feature = "dialoguer")]
use dialoguer::theme::ColorfulTheme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Confirm {
    #[serde(flatten)]
    pub common: CommonFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for Confirm {
    type Value = bool;
    fn dialoguer_value(&self) -> anyhow::Result<Self::Value> {
        if self.default.is_none() {
            Ok(dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(self.common.prompt.as_str())
                .interact()?)
        } else {
            Ok(dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(self.common.prompt.as_str())
                .default(self.default.unwrap())
                .interact()?)
        }
    }
}
