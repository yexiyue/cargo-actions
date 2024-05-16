#[macro_export]
macro_rules! error {
    ($($value:expr),*) => {
        println!("{}",console::style(format!("✘ {}",format!($($value),*))).red());
    };
}

#[macro_export]
macro_rules! success {
    ($($value:expr),*) => {
        println!("{}",console::style(format!("✔ {}",format!($($value),*))).green());
    };
}

#[macro_export]
macro_rules! info {
    ($($value:expr),*) => {
        println!("{}",console::style(format!($($value),*)).blue());
    };
}

#[macro_export]
macro_rules! warn {
    ($($value:expr),*) => {
       println!("{}",console::style(format!("⚠️ {}",format!($($value),*))).yellow());
    };
}
