mod capacitor;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::config::Config;
use crate::error::*;
use crate::symbol::Symbol;

use capacitor::CapacitorSymbol;

pub trait SymbolHandler {
    fn draw(&self, comp_cfg: &Config, lib_cfg: &Config) -> Result<Symbol>;
}

impl Debug for dyn SymbolHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymbolHandler")
    }
}

#[derive(Debug)]
pub struct Symbols {
    handlers: HashMap<&'static str, Box<dyn SymbolHandler>>,
}

impl Symbols {
    /// Creates an empty `Symbols`.
    pub fn new() -> Symbols {
        let mut handlers: HashMap<&'static str, Box<dyn SymbolHandler>> = HashMap::new();
        handlers.insert("capacitor", Box::new(CapacitorSymbol::new()));

        Symbols { handlers }
    }

    pub fn get_handler(&self, key: &str) -> Result<&dyn SymbolHandler> {
        self.handlers
            .get(key)
            .map(|v| v.as_ref())
            .ok_or_else(|| QedaError::InvalidSymbolType(key.to_string()).into())
    }
}

impl Default for Symbols {
    /// Creates an empty `Symbols`.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
