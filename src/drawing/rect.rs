use super::{Layer, Line, Point, Transform, Transformation};

#[derive(Clone, Default, Debug)]
pub struct Rect {
    pub p: (Point, Point),
    pub line_width: f64,
    pub layer: Layer,
}

impl Rect {
    /// Creates a new `Rect`.
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Rect {
            p: (
                Point::new(x0.min(x1), y0.min(y1)),
                Point::new(x0.max(x1), y0.max(y1)),
            ),
            line_width: 0.0,
            layer: Layer::NONE,
        }
    }

    /// Expands the `Rect` to the given delta.
    pub fn expand(mut self, d: f64) -> Self {
        self.p.0.x -= d;
        self.p.0.y -= d;
        self.p.1.x += d;
        self.p.1.y += d;
        self
    }

    /// Builds a `Rect` with modified layer.
    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }

    /// Converts the `Rect` to a vector of `Line`s.
    pub fn to_lines(&self) -> Vec<Line> {
        vec![
            Line::new(self.p.0.x, self.p.0.y, self.p.0.x, self.p.1.y)
                .width(self.line_width)
                .layer(self.layer),
            Line::new(self.p.0.x, self.p.1.y, self.p.1.x, self.p.1.y)
                .width(self.line_width)
                .layer(self.layer),
            Line::new(self.p.1.x, self.p.1.y, self.p.1.x, self.p.0.y)
                .width(self.line_width)
                .layer(self.layer),
            Line::new(self.p.1.x, self.p.0.y, self.p.0.x, self.p.0.y)
                .width(self.line_width)
                .layer(self.layer),
        ]
    }

    /// Builds a `Rect` with modified line width.
    #[inline]
    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }
}

impl Transform for Rect {
    fn transform(mut self, t: &Transformation) -> Self {
        self.line_width *= t.scale;
        self.p = (self.p.0.transform(t), self.p.1.transform(t));
        self
    }
}
