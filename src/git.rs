use anyhow::Result;
use dialoguer::{Input, Password};
use git2::Progress;
use std::{error::Error, fmt::Display, path::Path};

struct CredentialUI;

impl git2_credentials::CredentialUI for CredentialUI {
    fn ask_user_password(&self, username: &str) -> Result<(String, String), Box<dyn Error>> {
        let user: String = Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .default(username.to_owned())
            .with_prompt("username")
            .interact()?;
        let password: String = Password::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("password (hidden)")
            .allow_empty_password(true)
            .interact()?;
        Ok((user, password))
    }

    fn ask_ssh_passphrase(&self, passphrase_prompt: &str) -> Result<String, Box<dyn Error>> {
        let passphrase: String = Password::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt(passphrase_prompt)
            .allow_empty_password(true)
            .interact()?;
        Ok(passphrase)
    }
}
type ProgressCallback<'a> = Option<Box<dyn for<'b> FnMut(Progress<'b>) + 'a>>;

pub fn clone_repo(url: &str, path: &Path, progress_cb: ProgressCallback) -> Result<()> {
    // 创建一个 RemoteCallbacks 对象
    let mut cb = git2::RemoteCallbacks::new();
    // 打开默认的 git 配置
    let git_config = git2::Config::open_default()?;
    // 创建一个 CredentialHandler 对象
    let mut ch =
        git2_credentials::CredentialHandler::new_with_ui(git_config, Box::new(CredentialUI));

    if progress_cb.is_some() {
        let mut progress_cb = progress_cb.unwrap();
        cb.transfer_progress(move |progress| {
            progress_cb(progress);
            true
        });
    }

    // 设置凭证回调函数
    cb.credentials(move |url, username, allowed| ch.try_next_credential(url, username, allowed));

    // 创建一个 FetchOptions 对象
    let mut fo = git2::FetchOptions::new();
    // 设置远程回调函数
    fo.remote_callbacks(cb)
        // 下载所有标签
        .download_tags(git2::AutotagOption::All)
        // 更新 fetchhead
        .update_fetchhead(true);

    // 创建一个 RepoBuilder 对象
    git2::build::RepoBuilder::new()
        // 设置分支为 "master"
        .branch("master")
        // 设置 fetch options
        .fetch_options(fo)
        // 克隆仓库
        .clone(url, path)?;

    Ok(())
}

pub enum GitUrl {
    Http(String),
    Ssh(String),
    Unknown(String),
}
impl Display for GitUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitUrl::Http(url) => f.write_str(url),
            GitUrl::Ssh(url) => f.write_str(url),
            GitUrl::Unknown(url) => f.write_str(url),
        }
    }
}

impl<T: AsRef<str>> From<T> for GitUrl {
    fn from(value: T) -> Self {
        let value = value.as_ref().trim();
        if value.starts_with("https://github.com/") {
            GitUrl::Http(value.to_string())
        } else if value.starts_with("git@github.com:") {
            GitUrl::Ssh(value.to_string())
        } else {
            if value.starts_with("git@") || value.starts_with("https://") {
                return GitUrl::Unknown(value.to_string());
            }
            if value.ends_with(".git") {
                // https://mirror.ghproxy.com/https://github.com/
                return GitUrl::Unknown(format!("git@github.com:{value}"));
            }
            GitUrl::Unknown(format!("git@github.com:{value}.git"))
        }
    }
}

impl GitUrl {
    pub fn clone(&self, path: &Path) -> anyhow::Result<()> {
        match self {
            GitUrl::Http(url) => clone_repo(url, path, None),
            GitUrl::Ssh(url) => clone_repo(url, path, None),
            GitUrl::Unknown(url) => clone_repo(url, path, None),
        }
    }
}
