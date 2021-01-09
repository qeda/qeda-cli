use std::collections::HashMap;

use crate::config::Config;

#[derive(Debug)]
pub struct Outlines {
    outlines: HashMap<&'static str, Config>,
}

impl Outlines {
    /// Creates an empty `Outlines`.
    pub fn new() -> Self {
        let mut outlines = HashMap::new();

        outlines.insert(
            "jedec/ms-012",
            Config::from_yaml(include_str!("jedec/ms-012.yml")).unwrap(),
        );

        Self { outlines }
    }

    /// Gets the `Outline` addressed by the key.
    pub fn get(&self, key: &str) -> Option<&Config> {
        self.outlines.get(key)
    }
}

impl Default for Outlines {
    fn default() -> Self {
        Self::new()
    }
}
