use dialoguer::theme;
use git2;
use indicatif::{ProgressBar, ProgressStyle};
use std::{ffi::OsStr, fs::File, time::Duration};
use tempfile::tempdir;
use walkdir;

use crate::config::Config;

pub fn init(name: Option<String>) -> anyhow::Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["/", "|", "\\", "-", "/"])
            .template("{spinner:.green} {msg:.blue} {elapsed:>10.yellow}")?,
    );
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("🚀 Cloning...");
    let git_name = name.unwrap();
    let url = format!("https://github.com/{git_name}.git");
    let dir = tempdir()?;
    git2::Repository::clone(&url, dir.path())?;
    let entries = walkdir::WalkDir::new(dir.path());
    let mut cargo_actions = vec![];
    for entry in entries.into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == OsStr::new("cargo-action.json") {
            cargo_actions.push(entry.path().to_path_buf());
        }
    }
    let mut configs: Vec<Config> = vec![];
    for path in &cargo_actions {
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;
        configs.push(config);
    }
    spinner.finish_and_clear();
    let result = cargo_actions.iter().zip(configs.iter()).collect::<Vec<_>>();

    let choice = dialoguer::Select::with_theme(&theme::ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&configs)
        .default(0)
        .interact()?;
    let (action_path, config) = result[choice];
    let wirte_path = action_path.clone();
    config.write(&wirte_path.parent().unwrap().to_path_buf())?;
    Ok(())
}
