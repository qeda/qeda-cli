mod calc;
mod chip;
mod two_pin;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;

pub use two_pin::TwoPin;

use chip::ChipPackage;

pub trait PackageHandler {
    fn draw_pattern(&self, config: &Config) -> Result<Drawing>;
    fn draw_model(&self, config: &Config) -> Result<Drawing>;
}

impl Debug for dyn PackageHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackageHandler")
    }
}

#[derive(Debug)]
pub struct Packages<'a> {
    handlers: HashMap<&'a str, Box<dyn PackageHandler>>,
}

impl<'a> Packages<'a> {
    pub fn new() -> Self {
        let mut handlers: HashMap<&'a str, Box<dyn PackageHandler>> = HashMap::new();
        handlers.insert("chip", Box::new(ChipPackage::new()));

        Packages { handlers }
    }

    pub fn get_handler(&self, key: &str) -> Result<&Box<dyn PackageHandler>> {
        self.handlers
            .get(key)
            .ok_or(QedaError::InvalidPackageType(key.to_string()).into())
    }
}
