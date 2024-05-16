#[cfg(feature = "dialoguer")]
use super::DialoguerValue;
use super::{select::ItemOption, CommonFields};
#[cfg(feature = "dialoguer")]
use dialoguer::theme::ColorfulTheme;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiSelect {
    #[serde(flatten)]
    pub common: CommonFields,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<usize>>,
    pub options: Vec<ItemOption>,
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for MultiSelect {
    type Value = Value;

    fn dialoguer_value(&self) -> anyhow::Result<Self::Value> {
        let selected = if self.default.is_none() {
            dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.common.prompt)
                .items(&self.options)
                .interact()?
        } else {
            let len = self.options.len();
            let mut defaults = vec![false; len];
            for i in self.default.as_ref().unwrap() {
                defaults[*i] = true;
            }
            dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.common.prompt)
                .items(&self.options)
                .defaults(&defaults)
                .interact()?
        };
        Ok(selected
            .iter()
            .map(|item| self.options[*item].value.clone())
            .collect())
    }
}
