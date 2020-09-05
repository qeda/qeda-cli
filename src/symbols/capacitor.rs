use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;
use crate::pinout::*;
use crate::symbols::SymbolHandler;

pub struct CapacitorSymbol {}

impl CapacitorSymbol {
    pub fn new() -> CapacitorSymbol {
        CapacitorSymbol {}
    }
}

impl SymbolHandler for CapacitorSymbol {
    fn draw(&self, config: &Config) -> Result<Drawing> {
        debug!("draw capacitor symbol");

        let mut pinout = Pinout::from_config(config)?;
        if !pinout.groups.contains_key("L") {
            pinout.add_pin(Pin::new("L", "1").kind(PinKind::PASSIVE));
        }
        if !pinout.groups.contains_key("R") {
            pinout.add_pin(Pin::new("R", "2").kind(PinKind::PASSIVE));
        }

        let mut drawing = Drawing::from_svg(include_str!("capacitor.svg"), pinout)?;
        drawing.add_attr("ref-des", "C");

        Ok(drawing)
    }
}
