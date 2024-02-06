use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use std::fs::{self};
use toml::Table;

pub fn read_package_name() -> Result<String> {
    let toml_str = fs::read_to_string(std::env::current_dir().unwrap().join("Cargo.toml"))?;
    let table = toml::from_str::<Table>(&toml_str)?;
    let package = table.get("package").expect("package not found");
    let name = package.get("name").expect("package.name not found");
    Ok(name.as_str().unwrap().to_string())
}

pub fn read_members() -> Result<Vec<String>> {
    let package_name = read_package_name()?;
    let toml_str = fs::read_to_string(std::env::current_dir().unwrap().join("Cargo.toml"))?;
    let table = toml::from_str::<Table>(&toml_str)?;
    let workspace = table.get("workspace");
    if let Some(workspace) = workspace {
        if let Some(members) = workspace.get("members") {
            let mut members: Vec<String> = members
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().split("/").last().unwrap().to_string())
                .collect();
            members.push(package_name);
            Ok(members)
        } else {
            Ok(vec![package_name])
        }
    } else {
        Ok(vec![package_name])
    }
}

pub fn multiselect<'a, 'b, T: ToString>(options: &'a [T], prompt: &'b str) -> Result<Vec<&'a T>> {
    let defaults = vec![true; options.len()];
    let selected = dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
        .items(options)
        .with_prompt(prompt)
        .defaults(&defaults)
        .interact()?;
    Ok(selected.iter().map(|i| &options[*i]).collect::<Vec<_>>())
}
