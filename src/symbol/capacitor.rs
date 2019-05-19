use crate::errors::*;
use crate::config::Config;
use crate::symbol::SymbolHandler;
use crate::drawing::Drawing;

pub struct CapacitorSymbol {}

impl SymbolHandler for CapacitorSymbol {
    fn draw(&self, _config: &Config)-> Result<Drawing> {
        debug!("draw capacitor symbol");
        let mut drawing = Drawing::new();
        drawing.add_line(1, 2, 3, 4);
        Ok(drawing)
    } 
}

impl CapacitorSymbol {
    pub fn new()-> CapacitorSymbol {
        CapacitorSymbol {}
    } 
}
