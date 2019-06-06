pub trait Transform {
    fn transform(&mut self, t: &Transformation);
}

#[derive(Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Transform for Point {
    fn transform(&mut self, t: &Transformation) {
        t.transform(self);
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub p: (Point, Point)
}

impl Transform for Line {
    fn transform(&mut self, t: &Transformation) {
        self.p.0.transform(t);
        self.p.1.transform(t);
    }
}

#[derive(Debug)]
pub struct Transformation {
    m: [f64; 9],
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            m: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ]
        }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        let s = [
            sx,  0.0, 0.0,
            0.0, sy,  0.0,
            0.0, 0.0, 1.0,
        ];
        self.multiply(&s);
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        let t = [
            1.0, 0.0, dx,
            0.0, 1.0, dy,
            0.0, 0.0, 1.0,
        ];
        self.multiply(&t);
    }

    pub fn transform(&self, p: &mut Point) {
        let x = self.m[0]*p.x + self.m[1]*p.y + self.m[2];
        let y = self.m[3]*p.x + self.m[4]*p.y + self.m[5];
        p.x = x;
        p.y = y;
    }

    fn multiply(&mut self, n: &[f64; 9]) {
        let m00 = self.m[0]*n[0] + self.m[1]*n[3] + self.m[2]*n[6];
        let m01 = self.m[0]*n[1] + self.m[1]*n[4] + self.m[2]*n[7];
        let m02 = self.m[0]*n[2] + self.m[1]*n[5] + self.m[2]*n[8];
        let m10 = self.m[3]*n[0] + self.m[4]*n[3] + self.m[5]*n[6];
        let m11 = self.m[3]*n[1] + self.m[4]*n[4] + self.m[5]*n[7];
        let m12 = self.m[3]*n[2] + self.m[4]*n[5] + self.m[5]*n[8];
        let m20 = self.m[6]*n[0] + self.m[7]*n[3] + self.m[8]*n[6];
        let m21 = self.m[6]*n[1] + self.m[7]*n[4] + self.m[8]*n[7];
        let m22 = self.m[6]*n[2] + self.m[7]*n[5] + self.m[8]*n[8];
        self.m = [
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        ];
    }
}