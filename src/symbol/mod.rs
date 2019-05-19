mod capacitor;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::errors::*;
use crate::config::Config;
use crate::drawing::Drawing;

use capacitor::CapacitorSymbol;

pub trait SymbolHandler {
    fn draw(&self, config: &Config) -> Result<Drawing>;
}

impl Debug for SymbolHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SymbolHandler")
    }
}

#[derive(Debug)]
pub struct Symbol<'a> {
    handlers: HashMap<&'a str, Box<dyn SymbolHandler>>,
}

impl<'a> Symbol<'a> {
    pub fn new() -> Symbol<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn SymbolHandler>> = HashMap::new();
        handlers.insert("capacitor", Box::new(CapacitorSymbol::new()));

        Symbol {
            handlers,
        }
    }

    pub fn handler(&self, key: &str) -> Result<&Box<dyn SymbolHandler>> {
        self.handlers.get(key).ok_or(ErrorKind::InvalidSymbolHandler(key.to_string()).into())
    }

    pub fn draw(&self, config: &Config) -> Result<Drawing> {
        let symbol_handler = &config.get_string("symbol.handler")?;
        self.handler(&symbol_handler)?.draw(config)
    }
}
