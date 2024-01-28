use std::{ffi::OsStr, fs::File};
use git2;
use tempfile::tempdir;
use walkdir;

use crate::config::Config;

pub fn init(name: Option<String>) -> anyhow::Result<()> {
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
    let mut configs: Vec<Config> =vec![];
    for path in cargo_actions{
        let file = File::open(path)?;
        let config = serde_json::from_reader(file)?;
        configs.push(config);
    }

    println!("{:?}", configs);
    Ok(())
}
