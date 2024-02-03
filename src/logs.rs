#[macro_export]
macro_rules! error {
    ($($value:expr),*) => {
        tracing::error!("❌ {}",console::style(format!($($value),*)).red());
    };
}

#[macro_export]
macro_rules! success {
    ($($value:expr),*) => {
        tracing::info!("✅ {}",console::style(format!($($value),*)).green());
    };
}

#[macro_export]
macro_rules! info {
    ($($value:expr),*) => {
        tracing::info!("{}",console::style(format!($($value),*)).blue());
    };
}
