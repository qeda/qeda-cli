use crate::config::Config;
use crate::drawing::*;
use crate::error::*;

use super::{mask, silkscreen, PadProperties};

#[derive(Debug, Default)]
pub struct TwoPin {
    pad_props: PadProperties,
    body: Rect,
}

impl TwoPin {
    /// Builds a `TwoPin` with modified body.
    pub fn body(mut self, width: f64, height: f64) -> Self {
        self.body.p.0.x = -width / 2.0;
        self.body.p.0.y = -height / 2.0;
        self.body.p.1.x = width / 2.0;
        self.body.p.1.y = height / 2.0;
        self
    }

    /// Draws two pin pattern.
    pub fn draw(&self, drawing: &mut Drawing, lib_cfg: &Config) -> Result<()> {
        super::add_attributes(drawing, lib_cfg)?;

        let pad_left = Pad::new("1")
            .shape(PadShape::Rect)
            .size(self.pad_props.size.x, self.pad_props.size.y)
            .origin(-self.pad_props.distance / 2.0, 0.0)
            .layers(Layer::COPPER_TOP | Layer::MASK_TOP | Layer::PASTE_TOP);
        let pad_right = pad_left
            .clone()
            .name("2")
            .origin(self.pad_props.distance / 2.0, 0.0);

        let mut pads = vec![pad_left, pad_right];
        mask::calc(&mut pads, lib_cfg);
        silkscreen::draw_body(drawing, &self.body, &pads, lib_cfg);

        drawing.add_pads(pads);

        Ok(())
    }

    /// Builds a `TwoPin` with modified pad properties.
    pub fn pad_properties(mut self, pad_props: PadProperties) -> Self {
        self.pad_props = pad_props;
        self
    }
}
