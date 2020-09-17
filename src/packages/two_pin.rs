use crate::config::Config;
use crate::drawing::*;
use crate::error::*;

#[derive(Debug, Default)]
pub struct TwoPin {
    pub pad_size: (f64, f64),
    pub pad_distance: f64,
    pub courtyard: f64,
}

impl TwoPin {
    pub fn draw(&self, drawing: &mut Drawing, lib_config: &Config) -> Result<()> {
        let ref_des = Attribute::new("ref-des", "U")
            .font_size(lib_config.get_f64("pattern.font-size.ref-des")?)
            .line_width(lib_config.get_f64("pattern.line-width.silkscreen")?)
            .layer(Layer::SILKSCREEN_TOP);
        let value = Attribute::new("value", "?")
            .font_size(lib_config.get_f64("pattern.font-size.value")?)
            .line_width(lib_config.get_f64("pattern.line-width.assembly")?)
            .layer(Layer::ASSEMBLY_TOP);

        let pad_left = Pad::new("1")
            .shape(PadShape::Rect)
            .size(self.pad_size.0, self.pad_size.1)
            .origin(-self.pad_distance / 2.0, 0.0)
            .layers(Layer::COPPER_TOP | Layer::MASK_TOP | Layer::PASTE_TOP);
        let pad_right = pad_left
            .clone()
            .name("2")
            .origin(self.pad_distance / 2.0, 0.0);

        drawing.add_attribute(ref_des);
        drawing.add_attribute(value);
        drawing.add_pad(pad_left);
        drawing.add_pad(pad_right);
        Ok(())
    }
}
