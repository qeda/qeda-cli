#[macro_export]
macro_rules! load_config {
    ($yml:expr) => {
        crate::config::Config::from_str(include_str!($yml)).expect("failed to load config file")
    };
}
