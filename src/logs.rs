#[macro_export]
macro_rules! error {
    ($($value:expr),*) => {
        tracing::error!("{}",console::style(format!("✘ {}",format!($($value),*))).red());
    };
}

#[macro_export]
macro_rules! success {
    ($($value:expr),*) => {
        tracing::info!("{}",console::style(format!("✔ {}",format!($($value),*))).green());
    };
}

#[macro_export]
macro_rules! info {
    ($($value:expr),*) => {
        tracing::info!("{}",console::style(format!($($value),*)).blue());
    };
}

#[macro_export]
macro_rules! warn {
    ($($value:expr),*) => {
        tracing::warn!("{}",console::style(format!("⚠️ {}",format!($($value),*))).yellow());
    };
}
