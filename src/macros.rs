#[macro_export]
macro_rules! load_yaml {
    ($yml:expr) => (
        crate::YamlLoader::load_from_str(include_str!($yml)).expect("failed to load YAML file").pop().expect("invalid YAML file")
    );
}
