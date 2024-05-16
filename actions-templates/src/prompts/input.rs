#[cfg(feature = "dialoguer")]
use super::DialoguerValue;
#[cfg(feature = "dialoguer")]
use dialoguer::theme::ColorfulTheme;

use serde::{Deserialize, Serialize};

use super::CommonFields;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    #[serde(flatten)]
    pub common: CommonFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for Input {
    type Value = String;

    fn dialoguer_value(&self) -> anyhow::Result<Self::Value> {
        if self.default.is_none() {
            Ok(dialoguer::Input::with_theme(&ColorfulTheme::default())
                .with_prompt(self.common.prompt.as_str())
                .interact()?)
        } else {
            Ok(dialoguer::Input::with_theme(&ColorfulTheme::default())
                .with_prompt(self.common.prompt.as_str())
                .default(self.default.clone().unwrap())
                .interact()?)
        }
    }
}
