use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;

use super::PackageHandler;

pub struct SopPackage {}

impl SopPackage {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageHandler for SopPackage {
    fn draw_pattern(&self, _comp_cfg: &Config, _lib_cfg: &Config) -> Result<Drawing> {
        debug!("draw SOP pattern");
        let drawing = Drawing::new();
        Ok(drawing)
    }

    fn draw_model(&self, _comp_cfg: &Config, _lib_cfg: &Config) -> Result<Drawing> {
        debug!("draw SOP model");
        let drawing = Drawing::new();
        Ok(drawing)
    }
}
