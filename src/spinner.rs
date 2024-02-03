use indicatif::{ProgressBar, ProgressStyle};
use std::{ops::Deref, time::Duration};
use tracing::info;

pub struct Spinner(ProgressBar);

impl Deref for Spinner {
    type Target = ProgressBar;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Spinner {
    pub fn new() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["/", "|", "\\", "-", "/"])
                .template("{spinner:.green} {msg:.blue} {elapsed:>10.yellow}")
                .unwrap(),
        );
        Spinner(pb)
    }

    pub fn start(&self, msg: &'static str) {
        self.enable_steady_tick(Duration::from_millis(100));
        self.set_message(msg)
    }

    pub fn stop<F>(&self, msg: Option<F>)
    where
        F: Into<String>,
    {
        self.finish_and_clear();
        if msg.is_some() {
            info!("{}", msg.unwrap().into());
        }
    }
}
