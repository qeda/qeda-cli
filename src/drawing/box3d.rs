#[derive(Clone, Default, Debug)]
pub struct Box3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Box3D {
    /// Creates an empty `Box3D`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a `Box3D` with modified origin.
    pub fn origin(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    /// Builds a `Box3D` with modified dimensions.
    pub fn dimensions(mut self, dx: f64, dy: f64, dz: f64) -> Self {
        self.dx = dx;
        self.dy = dy;
        self.dz = dz;
        self
    }
}
