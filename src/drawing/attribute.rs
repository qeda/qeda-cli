use super::geometry::{Point, Transform, Transformation};
use super::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct Attribute {
    pub id: String,
    pub value: String,
    pub origin: Point,
    pub font_size: f64,
    pub halign: HAlign,
    pub valign: VAlign,
    pub orientation: Orientation,
    pub visibility: Visibility,
}

impl Attribute {
    pub fn new(id: &str, value: &str) -> Self {
        Attribute {
            id: id.to_string(),
            value: value.to_string(),
            origin: Point::default(),
            font_size: 0.0,
            halign: HAlign::default(),
            valign: VAlign::default(),
            orientation: Orientation::default(),
            visibility: Visibility(true),
        }
    }

    pub fn align(mut self, halign: HAlign, valign: VAlign) -> Self {
        self.halign = halign;
        self.valign = valign;
        self
    }

    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = font_size;
        self
    }

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
