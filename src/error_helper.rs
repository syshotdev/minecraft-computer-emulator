#[macro_export]
macro_rules! format_err {
    ($format:expr, $err:expr) => {
        Err(format!($format, $err))
    };
}

