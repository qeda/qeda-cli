use crate::drawing::*;

#[derive(Debug, Default)]
pub struct TwoPin {
    pub pad_size: (f64, f64),
    pub pad_distance: f64,
    pub courtyard: f64,
}

impl TwoPin {
    pub fn draw(&self, drawing: &mut Drawing) {
        let pad_left = Pad::new("1")
            .shape(PadShape::Rect)
            .size(self.pad_size.0, self.pad_size.1)
            .origin(-self.pad_distance / 2.0, 0.0)
            .layers(Layer::COPPER_TOP);
        let pad_right = pad_left
            .clone()
            .name("2")
            .origin(self.pad_distance / 2.0, 0.0);

        drawing.add_pad(pad_left);
        drawing.add_pad(pad_right);
    }
}
