use crate::errors::*;
use crate::config::Config;
use crate::patterns::PatternHandler;
use crate::drawing::Drawing;

pub struct ChipPattern {}

impl ChipPattern {
    pub fn new() -> ChipPattern {
        ChipPattern {}
    }
}

impl PatternHandler for ChipPattern {
    fn draw(&self, _config: &Config)-> Result<Drawing> {
        debug!("draw chip pattern");
        let mut drawing = Drawing::new();
        drawing.add_line(0.0, 1.0, 2.0, 3.0);
        Ok(drawing)
    } 
}
