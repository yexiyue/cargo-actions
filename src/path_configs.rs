use crate::{config::Config, utils::multiselect};
use anyhow::Context;
use dialoguer::theme;
use std::{borrow::Cow, fs::File, ops::Deref, path::PathBuf};

pub struct PathConfig<'a>(pub Cow<'a, PathBuf>, pub Cow<'a, Config>);

impl<'a> ToString for PathConfig<'a> {
    fn to_string(&self) -> String {
        self.1.title.clone()
    }
}

pub struct PathConfigs<'a> {
    inner: Vec<PathConfig<'a>>,
}

impl<'a> Deref for PathConfigs<'a> {
    type Target = Vec<PathConfig<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> PathConfigs<'a> {
    pub fn from(paths: &'a [PathBuf]) -> anyhow::Result<Self> {
        let mut configs = vec![];
        for path in paths {
            let file = File::open(path).with_context(|| format!("Failed to open {:?}", path))?;
            let config: Config = serde_json::from_reader(file)?;
            configs.push(config);
        }
        let path_configs = paths
            .iter()
            .zip(configs)
            .map(|(p, a)| PathConfig(Cow::Borrowed(p), Cow::Owned(a)))
            .collect::<Vec<_>>();
        Ok(Self {
            inner: path_configs,
        })
    }

    pub fn select(&self, prompt: &str) -> anyhow::Result<&PathConfig> {
        let index = dialoguer::Select::with_theme(&theme::ColorfulTheme::default())
            .with_prompt(prompt)
            .items(&self)
            .default(0)
            .interact()?;

        Ok(&self[index])
    }

    pub fn multiselect(&self, prompt: &str) -> anyhow::Result<Vec<&PathConfig>> {
        multiselect(&self, prompt)
    }
}
