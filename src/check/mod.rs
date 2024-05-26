use std::env::current_dir;

use actions_templates::ActionConfig;
use clap::Args;
use dialogue_macro::Asker;

use crate::{path_configs::WritePath, upload::UploadArgs, Run};

#[derive(Debug, Args, Asker)]
pub struct CheckArgs {
    /// Specifies the local path to the actions template folder to be checked.
    #[input(prompt = "请输入actions template文件夹路径：", with_default = true)]
    path: Option<String>,
}

impl Run for CheckArgs {
    fn run(&mut self) -> anyhow::Result<()> {
        if self.path.is_none() {
            let cur_dir = current_dir()?;
            let args = UploadArgs::asker().path(cur_dir.to_string_lossy()).finish();
            self.path = args.path;
        }
        let config = ActionConfig::from_dir(self.path.as_ref().unwrap())?;
        let default_write_path = current_dir()?
            .join(".github/workflows")
            .join(format!("{}.yaml", config.config.name))
            .to_string_lossy()
            .to_string();
        let write_path = WritePath::input_and_confirm(default_write_path)?;
        config.write_template(write_path)?;
        Ok(())
    }
}
