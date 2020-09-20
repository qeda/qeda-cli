mod chip;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;

use chip::ChipPackage;

#[derive(Debug)]
pub enum PackageType {
    Unknown,
    Chip,
}

impl Default for PackageType {
    #[inline]
    fn default() -> Self {
        PackageType::Unknown
    }
}

pub trait PackageHandler {
    fn draw_pattern(&self, comp_cfg: &Config, lib_cfg: &Config) -> Result<Drawing>;
    fn draw_model(&self, comp_cfg: &Config, lib_cfg: &Config) -> Result<Drawing>;
}

impl Debug for dyn PackageHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PackageHandler")
    }
}

#[derive(Debug)]
pub struct Packages {
    handlers: HashMap<&'static str, Box<dyn PackageHandler>>,
}

impl Packages {
    /// Creates an empty `Packages`.
    pub fn new() -> Self {
        let mut handlers: HashMap<&'static str, Box<dyn PackageHandler>> = HashMap::new();
        handlers.insert("chip", Box::new(ChipPackage::new()));

        Packages { handlers }
    }

    pub fn get_handler(&self, key: &str) -> Result<&dyn PackageHandler> {
        self.handlers
            .get(key)
            .map(|v| v.as_ref())
            .ok_or_else(|| QedaError::InvalidPackageType(key.to_string()).into())
    }
}

impl Default for Packages {
    /// Creates an empty `Packages`.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
