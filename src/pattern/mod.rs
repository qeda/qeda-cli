mod calc;
mod mask;
mod silkscreen;
mod two_pin;

use crate::config::Config;
use crate::drawing::{Attribute, Drawing, Layer, Size};
use crate::error::Result;

pub use calc::Ipc7351B;
pub use two_pin::TwoPin;

#[derive(Debug, Default)]
pub struct PadProperties {
    pub size: Size,
    pub distance: f64,
    pub courtyard: f64,
    lead_span: f64,
}

impl PadProperties {
    /// Applies post processing according to the pattern config.
    pub fn post_proc(mut self, comp_cfg: &Config, lib_cfg: &Config) -> Self {
        let space_for_iron = lib_cfg
            .get_f64("pattern.minimum.space-for-iron")
            .unwrap_or(0.0);
        let always_calc = lib_cfg
            .get_bool("pattern.always-calculate")
            .unwrap_or(false);

        if !always_calc {
            if let Some(pad_width) = comp_cfg.get_f64("pattern.pad-size-x").ok() {
                self.size.x = pad_width;
            }
            if let Some(pad_height) = comp_cfg.get_f64("pattern.pad-size-y").ok() {
                self.size.y = pad_height;
            }
            if let Some(pad_size) = comp_cfg.get_pair("pattern.pad-size").ok() {
                self.size = Size::new(pad_size.0, pad_size.1);
            }
            if let Some(pad_distance) = comp_cfg.get_f64("pattern.pad-distance").ok() {
                self.distance = pad_distance;
            }
            if let Some(pad_span) = comp_cfg.get_f64("pattern.pad-span").ok() {
                self.distance = pad_span - self.size.x;
            }
            if let Some(pad_space) = comp_cfg.get_f64("pattern.pad-space").ok() {
                self.distance = pad_space + self.size.x;
            }
        }
        if space_for_iron > 0.0 {
            let lead_to_pad = (self.distance + self.size.x - self.lead_span) / 2.0;
            if lead_to_pad < space_for_iron {
                let d = space_for_iron - lead_to_pad;
                self.size.x += d;
                self.distance += d;
            }
        }
        self
    }
}

fn add_attributes(drawing: &mut Drawing, lib_cfg: &Config) -> Result<()> {
    let ref_des = Attribute::new("ref-des", "U")
        .font_size(lib_cfg.get_f64("pattern.font-size.ref-des")?)
        .line_width(lib_cfg.get_f64("pattern.line-width.silkscreen")?)
        .layer(Layer::SILKSCREEN_TOP);
    let value = Attribute::new("value", "?")
        .font_size(lib_cfg.get_f64("pattern.font-size.value")?)
        .line_width(lib_cfg.get_f64("pattern.line-width.assembly")?)
        .layer(Layer::ASSEMBLY_TOP);

    drawing.add_attribute(ref_des);
    drawing.add_attribute(value);

    Ok(())
}
