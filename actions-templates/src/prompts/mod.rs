use self::{confirm::Confirm, input::Input, multiselect::MultiSelect, select::Select};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::ops::{Deref, DerefMut};
mod confirm;
mod input;
mod multiselect;
mod select;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Prompt {
    Confirm(Confirm),
    Input(Input),
    MultiSelect(MultiSelect),
    Select(Select),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonFields {
    pub prompt: String,
    pub field: String,
}

#[cfg(feature = "dialoguer")]
pub trait DialoguerValue {
    type Value;
    fn dialoguer_value(&self) -> Result<Self::Value>;
}

impl Prompt {
    pub fn get_field(&self) -> &str {
        match self {
            Prompt::Input(input) => &input.common.field,
            Prompt::Select(select) => &select.common.field,
            Prompt::Confirm(confirm) => &confirm.common.field,
            Prompt::MultiSelect(multiselect) => &multiselect.common.field,
        }
    }
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for Prompt {
    type Value = Value;
    fn dialoguer_value(&self) -> Result<Self::Value> {
        match self {
            Prompt::Input(input) => input.dialoguer_value().map(Value::String),
            Prompt::Select(select) => select.dialoguer_value(),
            Prompt::Confirm(confirm) => confirm.dialoguer_value().map(Value::Bool),
            Prompt::MultiSelect(multiselect) => multiselect.dialoguer_value(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompts(Vec<Prompt>);

impl Deref for Prompts {
    type Target = Vec<Prompt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prompts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "dialoguer")]
impl DialoguerValue for Prompts {
    type Value = Value;
    fn dialoguer_value(&self) -> Result<Value> {
        let mut map = Map::new();

        for prompt in self.iter() {
            map.insert(prompt.get_field().to_owned(), prompt.dialoguer_value()?);
        }

        Ok(Value::Object(map))
    }
}
