use crate::errors::*;
use crate::config::Config;
use crate::symbols::SymbolHandler;
use crate::drawing::Drawing;

pub struct CapacitorSymbol {}

impl CapacitorSymbol {
    pub fn new()-> CapacitorSymbol {
        CapacitorSymbol {}
    } 
}

impl SymbolHandler for CapacitorSymbol {
    fn draw(&self, _config: &Config)-> Result<Drawing> {
        debug!("draw capacitor symbol");
        let mut drawing = Drawing::new();
        drawing.add_line(1, 2, 3, 4);
        Ok(drawing)
    } 
}
