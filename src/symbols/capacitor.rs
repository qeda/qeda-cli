use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;
use crate::pinout::*;
use crate::symbol::Symbol;

use super::SymbolHandler;

pub struct CapacitorSymbol {}

impl CapacitorSymbol {
    pub fn new() -> Self {
        Self {}
    }
}

impl SymbolHandler for CapacitorSymbol {
    fn draw(&self, comp_cfg: &Config, _lib_cfg: &Config) -> Result<Symbol> {
        debug!("draw capacitor symbol");

        let mut pinout = Pinout::from_config(comp_cfg)?;
        if !pinout.groups.contains_key("L") {
            pinout.add_pin(Pin::new("L", "1").kind(PinKind::PASSIVE));
        }
        if !pinout.groups.contains_key("R") {
            pinout.add_pin(Pin::new("R", "2").kind(PinKind::PASSIVE));
        }

        let mut result = Symbol::new();
        result.add_part(Drawing::from_svg(include_str!("capacitor.svg"), pinout)?);
        Ok(result)
    }
}
