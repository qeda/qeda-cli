#[derive(Debug)]
pub struct Outlines {}

impl Outlines {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Outlines {
    fn default() -> Self {
        Self::new()
    }
}
