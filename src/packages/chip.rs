use crate::config::Config;
use crate::drawing::{Box3D, Drawing};
use crate::error::*;
use crate::pattern::{Ipc7351B, TwoPin};

use super::{PackageHandler, PackageType};

pub struct ChipPackage {}

impl ChipPackage {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageHandler for ChipPackage {
    fn draw_pattern(&self, comp_cfg: &Config, lib_cfg: &Config) -> Result<Drawing> {
        debug!("draw chip pattern");

        let body_size_x = comp_cfg.get_range("package.body-size-x")?;
        let body_size_y = comp_cfg.get_range("package.body-size-y")?;
        let body_width = body_size_x.nom();
        let body_height = body_size_y.nom();
        let body_size_z = if let Ok(z) = comp_cfg.get_range("package.body-size-z") {
            z
        } else if let Ok(z) = comp_cfg.get_range("package.size-z") {
            z
        } else {
            bail!(QedaError::MissingDimension(
                "'package' should have either 'body_size_z' or 'size_z'"
            ));
        };
        let lead_len = comp_cfg.get_range("package.lead-length")?;

        let pad_props = Ipc7351B::new(PackageType::Chip)
            .lead_span(body_size_x)
            .lead_width(body_size_y)
            .lead_height(body_size_z) // TODO: Check whether we really need it
            .lead_len(lead_len)
            .settings(lib_cfg)
            .calc()
            .post_proc(comp_cfg, lib_cfg);

        let two_pin = TwoPin::default()
            .pad_properties(pad_props)
            .body(body_width, body_height);

        let mut drawing = Drawing::new();
        two_pin.draw(&mut drawing, lib_cfg);
        Ok(drawing)
    }

    fn draw_model(&self, _comp_cfg: &Config, _lib_cfg: &Config) -> Result<Drawing> {
        debug!("draw chip model");
        let mut drawing = Drawing::new();
        drawing.add_box3d(Box3D::new().origin(0.0, 1.0, 2.0).dimensions(3.0, 4.0, 5.0));
        Ok(drawing)
    }
}
