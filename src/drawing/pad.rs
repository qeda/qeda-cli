use super::{Layer, Point, Size};

#[derive(Clone, Debug)]
pub enum PadShape {
    Circle,
    Rect,
    RoundRect,
}

impl Default for PadShape {
    fn default() -> Self {
        PadShape::Circle
    }
}

#[derive(Clone, Default, Debug)]
pub struct Pad {
    pub origin: Point,
    pub size: Size,
    pub shape: PadShape,
    pub hole: Option<Size>,
    pub layers: Layer,
}

impl Pad {
    pub fn layers(mut self, layers: Layer) -> Self {
        self.layers = layers;
        self
    }

    pub fn origin(mut self, x: f64, y: f64) -> Self {
        self.origin.x = x;
        self.origin.y = y;
        self
    }

    pub fn shape(mut self, shape: PadShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn size(mut self, x: f64, y: f64) -> Self {
        self.size.x = x;
        self.size.y = y;
        self
    }
}
