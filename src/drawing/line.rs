use super::{Layer, Point, Transform, Transformation};

#[derive(Clone, Default, Debug)]
pub struct Line {
    pub p: (Point, Point),
    pub width: f64,
    pub layer: Layer,
}

impl Line {
    /// Creates a new line.
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Line {
            p: (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }),
            width: 0.0,
            layer: Layer::NONE,
        }
    }

    /// Changes the layer.
    pub fn layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }

    /// Changes line width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    /// Returns line length.
    pub fn len(&self) -> f64 {
        Point::distance(&self.p.0, &self.p.1)
    }

    /// Returns maximum x-coordinate.
    #[inline]
    pub fn max_x(&self) -> f64 {
        self.p.0.x.max(self.p.1.x)
    }

    /// Returns maximum y-coordinate.
    #[inline]
    pub fn max_y(&self) -> f64 {
        self.p.0.y.max(self.p.1.y)
    }

    /// Returns minimum x-coordinate.
    #[inline]
    pub fn min_x(&self) -> f64 {
        self.p.0.x.min(self.p.1.x)
    }

    /// Returns minimum y-coordinate.
    #[inline]
    pub fn min_y(&self) -> f64 {
        self.p.0.y.min(self.p.1.y)
    }
}

impl Transform for Line {
    fn transform(mut self, t: &Transformation) -> Self {
        self.width *= t.scale;
        self.p = (self.p.0.transform(t), self.p.1.transform(t));
        self
    }
}
