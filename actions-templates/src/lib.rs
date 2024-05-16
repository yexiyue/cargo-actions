use anyhow::Result;
use config::Config;
use prompts::DialoguerValue;
use readme::ReadME;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use template::Template;

pub mod config;
pub mod prompts;
pub mod readme;
pub mod template;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActionConfig {
    pub config: Config,
    /// 完整的描述，包括一些配置信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<ReadME>,
    pub template: Template,
}

impl ActionConfig {
    // 标准Cargo actions模版目录
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self> {
        // 读取配置文件
        let config_path = path.as_ref().join("cargo-action.json");
        let config = Config::from_file(config_path)?;
        // 读取 README.md 文件
        let readme_path = path.as_ref().join("README.md");
        let readme = if readme_path.exists() {
            Some(ReadME::from_file(readme_path)?)
        } else {
            None
        };

        // 读取模版文件
        let template_path = path.as_ref().join(
            &config
                .path
                .as_ref()
                .unwrap_or(&format!("{}.yaml.hbs", &config.name)),
        );
        let template = Template::from_file(template_path)?;

        Ok(Self {
            config,
            readme,
            template,
        })
    }

    #[cfg(feature = "dialoguer")]
    pub fn write_template<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = self.config.prompts.dialoguer_value()?;
        let contents = self.template.render_to_string(&data)?;
        Ok(fs::write(path, contents)?)
    }
}
