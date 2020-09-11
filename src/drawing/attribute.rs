use super::prelude::*;
use super::{Layer, Point, Transform, Transformation};

#[derive(Clone, Default, Debug)]
pub struct Attribute {
    pub id: String,
    pub value: String,
    pub origin: Point,
    pub font_size: f64,
    pub line_width: f64,
    pub halign: HAlign,
    pub valign: VAlign,
    pub orientation: Orientation,
    pub layer: Layer,
    pub visibility: Visibility,
}

impl Attribute {
    /// Creates a new `Attribute`.
    ///
    /// Attribute is a text box with ID and drawing properties.
    pub fn new(id: &str, value: &str) -> Self {
        Attribute {
            id: id.to_string(),
            value: value.to_string(),
            origin: Point::default(),
            font_size: 0.0,
            line_width: 0.0,
            halign: HAlign::default(),
            valign: VAlign::default(),
            orientation: Orientation::default(),
            layer: Layer::NONE,
            visibility: Visibility(true),
        }
    }

    /// Builds an `Attribute` with modified alignment.
    pub fn align(mut self, halign: HAlign, valign: VAlign) -> Self {
        self.halign = halign;
        self.valign = valign;
        self
    }

    /// Builds an `Attribute` with modified font size.
    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = font_size;
        self
    }

    /// Builds an `Attribute` with modified layer.
    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }

    /// Builds an `Attribute` with modified line width.
    pub fn line_width(mut self, line_width: f64) -> Self {
        self.line_width = line_width;
        self
    }

    /// Builds an `Attribute` with modified origin.
    pub fn origin(mut self, x: f64, y: f64) -> Self {
        self.origin = Point::new(x, y);
        self
    }
}

impl Transform for Attribute {
    fn transform(mut self, t: &Transformation) -> Self {
        self.origin = self.origin.transform(t);
        self.font_size *= t.scale;
        // TODO: Change `orientation` depending on rotation
        self
    }
}
