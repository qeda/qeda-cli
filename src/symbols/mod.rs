mod capacitor;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;

use capacitor::CapacitorSymbol;

pub trait SymbolHandler {
    fn draw(&self, config: &Config) -> Result<Drawing>;
}

impl Debug for dyn SymbolHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SymbolHandler")
    }
}

#[derive(Debug)]
pub struct Symbols<'a> {
    handlers: HashMap<&'a str, Box<dyn SymbolHandler>>,
}

impl<'a> Symbols<'a> {
    pub fn new() -> Symbols<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn SymbolHandler>> = HashMap::new();
        handlers.insert("capacitor", Box::new(CapacitorSymbol::new()));

        Symbols { handlers }
    }

    pub fn get_handler(&self, key: &str) -> Result<&Box<dyn SymbolHandler>> {
        self.handlers
            .get(key)
            .ok_or(QedaError::InvalidSymbolType(key.to_string()).into())
    }
}
