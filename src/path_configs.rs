use actions_templates::ActionConfig;
use anyhow::anyhow;
use dialogue_macro::Asker;
use std::{env::current_dir, ops::Deref, path::PathBuf};

#[derive(Debug, Clone)]
pub struct PathConfig(ActionConfig);

impl Deref for PathConfig {
    type Target = ActionConfig;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for PathConfig {
    fn to_string(&self) -> String {
        self.0.config.name.clone()
    }
}

pub struct PathConfigs {
    inner: Vec<PathConfig>,
}

impl Deref for PathConfigs {
    type Target = Vec<PathConfig>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PathConfigs {
    pub fn from(paths: &[PathBuf]) -> anyhow::Result<Self> {
        let mut configs = vec![];
        for path in paths {
            let config = ActionConfig::from_dir(path)?;
            configs.push(config);
        }
        let path_configs = configs
            .into_iter()
            .map(|a| PathConfig(a))
            .collect::<Vec<_>>();
        Ok(Self {
            inner: path_configs,
        })
    }
}

#[derive(Debug, Clone, Asker)]
pub struct SelectPathConfig {
    #[select(with_default = true, prompt = "请选择actions template")]
    pub action_config: PathConfig,
}

#[derive(Debug, Clone, Asker)]
pub struct WritePath {
    #[input(with_default = true, prompt = "请输入写入路径：")]
    pub write_path: String,
}

impl AsRef<str> for WritePath {
    fn as_ref(&self) -> &str {
        self.write_path.as_ref()
    }
}

impl SelectPathConfig {
    pub fn write_template(&self) -> anyhow::Result<()> {
        let default_write_path = current_dir()?
            .join(".github/workflows")
            .join(format!("{}.yaml", self.action_config.config.name))
            .to_string_lossy()
            .to_string();
        let write_path = WritePath::asker().write_path(default_write_path).finish();
        self.action_config.write_template(write_path.as_ref());
        Ok(())
    }
}
