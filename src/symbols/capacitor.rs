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
        let mut drawing = Drawing::from_svg(include_str!("capacitor.svg"))?;
        drawing.add_attr("ref-des", "C");
        Ok(drawing)
    }
}
