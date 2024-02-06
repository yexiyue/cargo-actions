use dialoguer::{self, theme::ColorfulTheme};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::Runner;

/// prompts字段
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Prompt {
    #[serde(rename = "input")]
    Input(Input),
    #[serde(rename = "select")]
    Select(Select),
    #[serde(rename = "confirm")]
    Confirm(Confirm),
    #[serde(rename = "multiselect")]
    MultiSelect(MultiSelect),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemOption {
    label: String,
    value: Value,
}

impl ToString for ItemOption {
    fn to_string(&self) -> String {
        self.label.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    prompt: String,
    field: String,
    default: Option<String>,
}

impl Runner for Input {
    type Output = Value;
    fn run(&self) -> anyhow::Result<Self::Output> {
        if self.default.is_none() {
            Ok(Value::String(
                dialoguer::Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(self.prompt.as_str())
                    .interact()?,
            ))
        } else {
            Ok(Value::String(
                dialoguer::Input::with_theme(&ColorfulTheme::default())
                    .with_prompt(self.prompt.as_str())
                    .default(self.default.clone().unwrap())
                    .interact()?,
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Select {
    prompt: String,
    field: String,
    default: Option<usize>,
    options: Vec<ItemOption>,
}

impl Runner for Select {
    type Output = Value;

    fn run(&self) -> anyhow::Result<Self::Output> {
        let selected = if self.default.is_none() {
            dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.prompt)
                .items(&self.options)
                .interact()?
        } else {
            dialoguer::Select::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.prompt)
                .items(&self.options)
                .default(self.default.unwrap())
                .interact()?
        };
        Ok(self.options[selected].value.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Confirm {
    prompt: String,
    field: String,
    default: Option<bool>,
}

impl Runner for Confirm {
    type Output = Value;
    fn run(&self) -> anyhow::Result<Self::Output> {
        if self.default.is_none() {
            Ok(Value::Bool(
                dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(self.prompt.as_str())
                    .interact()?,
            ))
        } else {
            Ok(Value::Bool(
                dialoguer::Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(self.prompt.as_str())
                    .default(self.default.clone().unwrap())
                    .interact()?,
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiSelect {
    prompt: String,
    field: String,
    default: Option<Vec<usize>>,
    options: Vec<ItemOption>,
}

impl Runner for MultiSelect {
    type Output = Value;

    fn run(&self) -> anyhow::Result<Self::Output> {
        let selected = if self.default.is_none() {
            dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.prompt)
                .items(&self.options)
                .interact()?
        } else {
            let len = self.options.len();
            let mut defaults = vec![false; len];
            for i in &self.default.clone().unwrap() {
                defaults[*i] = true;
            }
            dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
                .with_prompt(&self.prompt)
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

impl Runner for Prompt {
    type Output = Value;

    fn run(&self) -> anyhow::Result<Self::Output> {
        match self {
            Prompt::Input(input) => input.run(),
            Prompt::Select(select) => select.run(),
            Prompt::Confirm(confirm) => confirm.run(),
            Prompt::MultiSelect(multiselect) => multiselect.run(),
        }
    }
}

impl Prompt {
    pub fn get_field(&self) -> String {
        match self {
            Prompt::Input(input) => input.field.clone(),
            Prompt::Select(select) => select.field.clone(),
            Prompt::Confirm(confirm) => confirm.field.clone(),
            Prompt::MultiSelect(multiselect) => multiselect.field.clone(),
        }
    }
}
