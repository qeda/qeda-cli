use std::marker::Sized;

pub trait Transform {
    fn transform(self, t: &Transformation) -> Self;

    fn scale(self, sx: f64, sy: f64) -> Self
    where
        Self: Sized,
    {
        let mut t = Transformation::new();
        t.scale(sx, sy);
        self.transform(&t)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Size {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }
    pub fn distance(begin: &Point, end: &Point) -> f64 {
        ((end.x - begin.x).powi(2) + (end.y - begin.y).powi(2)).sqrt()
    }
}

impl Transform for Point {
    fn transform(mut self, t: &Transformation) -> Self {
        t.transform_point(&mut self);
        self
    }
}

#[derive(Clone, Default, Debug)]
pub struct Line {
    pub p: (Point, Point),
    pub width: f64,
}

impl Line {
    /// Creates a new line.
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Line {
            p: (Point { x: x0, y: y0 }, Point { x: x1, y: y1 }),
            width: 0.0,
        }
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

#[derive(Debug)]
pub struct Transformation {
    pub scale: f64,
    pub scale_x: f64,
    pub scale_y: f64,

    m: [f64; 9],
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        let s = [sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0];
        self.multiply(&s);
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        let t = [1.0, 0.0, dx, 0.0, 1.0, dy, 0.0, 0.0, 1.0];
        self.multiply(&t);
    }

    pub fn transform_point(&self, p: &mut Point) {
        let x = self.m[0] * p.x + self.m[1] * p.y + self.m[2];
        let y = self.m[3] * p.x + self.m[4] * p.y + self.m[5];
        p.x = x;
        p.y = y;
    }

    fn multiply(&mut self, n: &[f64; 9]) {
        let m00 = n[0] * self.m[0] + n[1] * self.m[3] + n[2] * self.m[6];
        let m01 = n[0] * self.m[1] + n[1] * self.m[4] + n[2] * self.m[7];
        let m02 = n[0] * self.m[2] + n[1] * self.m[5] + n[2] * self.m[8];
        let m10 = n[3] * self.m[0] + n[4] * self.m[3] + n[5] * self.m[6];
        let m11 = n[3] * self.m[1] + n[4] * self.m[4] + n[5] * self.m[7];
        let m12 = n[3] * self.m[2] + n[4] * self.m[5] + n[5] * self.m[8];
        let m20 = n[6] * self.m[0] + n[7] * self.m[3] + n[8] * self.m[6];
        let m21 = n[6] * self.m[1] + n[7] * self.m[4] + n[8] * self.m[7];
        let m22 = n[6] * self.m[2] + n[7] * self.m[5] + n[8] * self.m[8];
        self.m = [m00, m01, m02, m10, m11, m12, m20, m21, m22];
        self.scale_x = (self.m[0] * self.m[0] + self.m[3] * self.m[3]).sqrt();
        self.scale_y = (self.m[1] * self.m[1] + self.m[4] * self.m[4]).sqrt();
        self.scale = if self.scale_x == self.scale_y {
            self.scale_x
        } else {
            ((self.scale_x * self.scale_x + self.scale_y * self.scale_y) / 2.0).sqrt()
        }
    }
}
