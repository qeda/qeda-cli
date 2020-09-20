use super::*;

#[derive(Clone, Debug)]
pub enum PadShape {
    Circle,
    Rect,
    RoundRect,
}

impl Default for PadShape {
    #[inline]
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
    pub mask: f64,
}

impl Pad {
    /// Creates an empty `Pad` with specified name.
    pub fn new(name: &str) -> Self {
        Pad {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Returns `true` if `Pad` has surface mount type.
    #[inline]
    pub fn is_smd(&self) -> bool {
        self.hole.is_none()
    }

    /// Builds a `Pad` with modified layers.
    #[inline]
    pub fn layers(mut self, layers: Layer) -> Self {
        self.layers = layers;
        self
    }

    /// Builds a `Pad` with modified name.
    #[inline]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Builds a `Pad` with modified origin.
    pub fn origin(mut self, x: f64, y: f64) -> Self {
        self.origin.x = x;
        self.origin.y = y;
        self
    }

    /// Builds a `Pad` with modified shape.
    #[inline]
    pub fn shape(mut self, shape: PadShape) -> Self {
        self.shape = shape;
        self
    }

    /// Builds a `Pad` with modified size.
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
