use crate::errors::*;
use crate::config::Config;
use crate::pattern::PatternHandler;
use crate::drawing::Drawing;

pub struct ChipPattern {}

impl PatternHandler for ChipPattern {
    fn draw(&self, _config: &Config)-> Result<Drawing> {
        debug!("draw chip pattern");
        let mut drawing = Drawing::new();
        drawing.add_line(0, 0, 0, 0);
        Ok(drawing)
    } 
}
