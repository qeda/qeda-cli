use super::*;

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
    pub name: String,
    pub origin: Point,
    pub size: Size,
    pub shape: PadShape,
    pub hole: Option<Size>,
    pub layers: Layer,
}

impl Pad {
    pub fn new(name: &str) -> Self {
        Self::default().name(name)
    }

    pub fn is_smd(&self) -> bool {
        self.hole.is_none()
    }

    pub fn layers(mut self, layers: Layer) -> Self {
        self.layers = layers;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
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

impl Transform for Pad {
    fn transform(mut self, t: &Transformation) -> Self {
        self.origin = self.origin.transform(t);
        self.size = self.size.transform(t);
        // TODO: Consider rotation
        self
    }
}
