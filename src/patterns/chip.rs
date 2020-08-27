use crate::config::Config;
use crate::drawing::Drawing;
use crate::errors::*;
use crate::patterns::PatternHandler;

pub struct ChipPattern {}

impl ChipPattern {
    pub fn new() -> ChipPattern {
        ChipPattern {}
    }
}

impl PatternHandler for ChipPattern {
    fn draw(&self, _config: &Config) -> Result<Drawing> {
        debug!("draw chip pattern");
        let mut drawing = Drawing::new();
        drawing.add_line(0.0, 1.0, 2.0, 3.0, 0.5);
        Ok(drawing)
    }
}
