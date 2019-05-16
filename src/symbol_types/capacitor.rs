use crate::errors::*;
use crate::config::Config;
use crate::symbol::SymbolType;
use crate::drawing::Drawing;

pub struct CapacitorSymbol {}

impl SymbolType for CapacitorSymbol {
    fn draw(&self, _config: &Config)-> Result<Drawing> {
        debug!("draw capacitor symbol");
        let mut drawing = Drawing::new();
        drawing.add_line(0, 0, 0, 0);
        Ok(drawing)
    } 
}