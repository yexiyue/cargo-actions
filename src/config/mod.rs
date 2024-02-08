use dialoguer::theme;
use handlebars::Context;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
mod prompt;
use crate::{info, success, utils::read_package_name};
use prompt::Prompt;
use serde_json::{json, Value};

/// 解析配置文件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub path: String,
    pub prompts: Vec<Prompt>,
    #[serde(rename = "finishTooltip")]
    pub finish_tooltip: String,
    pub author: Option<String>,
    pub description: String,
}

impl ToString for Config {
    fn to_string(&self) -> String {
        self.description.to_string()
    }
}

/// 定义runner接口，通过dialoguer获取相应的输入
pub trait Runner {
    type Output;
    fn run(&self) -> anyhow::Result<Self::Output>;
}

impl Runner for Config {
    type Output = Value;
    fn run(&self) -> anyhow::Result<Self::Output> {
        let mut value = json!({});
        for i in &self.prompts {
            let name = i.get_field();
            value[name] = i.run()?;
        }
        Ok(value)
    }
}

impl Config {
    pub fn write(&self, base_path: &PathBuf) -> anyhow::Result<()> {
        // 模版文件路径
        let path = base_path.join(&self.path);
        // 获取payload
        let mut ctx = Context::wraps(self.run()?)?;
        let package_name = read_package_name()?;
        ctx.data_mut()["package_name"] = Value::String(package_name);

        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_string_lossy().replace(".hbs", "");
        let file_name: String = dialoguer::Input::with_theme(&theme::ColorfulTheme::default())
            .with_prompt("Please enter the workflow file name")
            .default(file_name)
            .interact()?;

        // 读取模版文件内容
        let template_string = std::fs::read_to_string(path)?;
        let dir = std::env::current_dir()?.join(".github/workflows");
        fs_extra::dir::create_all(&dir, false)?;
        // 写入文件的路径
        let path_to_write = dir.join(file_name);

        match fs::metadata(&path_to_write) {
            Ok(_) => {
                let confirm = dialoguer::Confirm::with_theme(&theme::ColorfulTheme::default())
                    .with_prompt("Are you sure you want to overwrite this file?")
                    .interact()?;
                if confirm {
                    // 渲染模版内容，写入文件中。
                    let writer = fs::File::create(path_to_write)?;
                    let handlebar = handlebars::Handlebars::new();
                    handlebar.render_template_with_context_to_write(
                        &template_string,
                        &ctx,
                        writer,
                    )?;
                }
            }
            _ => {
                // 渲染模版内容，写入文件中。
                let writer = fs::File::create(path_to_write)?;
                let handlebar = handlebars::Handlebars::new();
                handlebar.render_template_with_context_to_write(&template_string, &ctx, writer)?;
            }
        }

        success!("Created successfully!\n");
        info!("{}", &self.finish_tooltip);
        Ok(())
    }

    pub fn from(value: &PathBuf) -> anyhow::Result<Self> {
        let file = fs::File::open(value)?;
        Ok(serde_json::from_reader(file)?)
    }
}
