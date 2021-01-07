use crate::config::Config;
use crate::error::*;
use crate::symbol::Symbol;

use super::SymbolHandler;

pub struct IcSymbol {}

impl IcSymbol {
    pub fn new() -> Self {
        Self {}
    }
}

impl SymbolHandler for IcSymbol {
    fn draw(&self, _comp_cfg: &Config, _lib_cfg: &Config) -> Result<Symbol> {
        debug!("draw IC symbol");

        let result = Symbol::new();
        Ok(result)
    }
}
