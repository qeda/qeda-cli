use crate::config::Config;
use crate::drawing::{Box3D, Drawing, Line};
use crate::error::*;

use super::PackageHandler;

pub struct ChipPackage {}

impl ChipPackage {
    pub fn new() -> Self {
        ChipPackage {}
    }
}

impl PackageHandler for ChipPackage {
    fn draw_pattern(&self, _config: &Config) -> Result<Drawing> {
        debug!("draw chip pattern");
        let mut drawing = Drawing::new();
        drawing.add_line(Line::new(0.0, 1.0, 2.0, 3.0).width(0.5));
        Ok(drawing)
    }

    fn draw_model(&self, _config: &Config) -> Result<Drawing> {
        debug!("draw chip model");
        let mut drawing = Drawing::new();
        drawing.add_box3d(Box3D::new().origin(0.0, 1.0, 2.0).dimensions(3.0, 4.0, 5.0));
        Ok(drawing)
    }
}
