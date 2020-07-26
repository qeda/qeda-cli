use crate::errors::*;
use crate::config::Config;
use crate::symbols::SymbolHandler;
use crate::symbols::pinout::Pinout;
use crate::drawing::Drawing;

pub struct CapacitorSymbol {}

impl CapacitorSymbol {
    pub fn new()-> CapacitorSymbol {
        CapacitorSymbol {}
    }
}

impl SymbolHandler for CapacitorSymbol {
    fn draw(&self, config: &Config)-> Result<Drawing> {
        debug!("draw capacitor symbol");

        let mut drawing = Drawing::from_svg(include_str!("capacitor.svg"))?;
        drawing.add_attr("ref-des", "C");

        let mut pinout = Pinout::from_config(config);
        pinout.add_default("L", 1);
        pinout.add_default("R", 2);
        pinout.apply_to(drawing.mut_elements());

        Ok(drawing)
    }
}
