use crate::errors::*;
use crate::config::Config;
use crate::drawing::Drawing;

use crate::symbol_types:: {
    capacitor::CapacitorSymbol
};

pub trait SymbolType {
    fn draw(&self, config: &Config) -> Result<Drawing>;
}

pub struct Symbol {
    symbol_type: Box<dyn SymbolType>,
    drawing: Drawing,
}

impl Symbol {
    pub fn from(config: &Config) -> Result<Symbol>{
        let symbol_type = config.get_string("symbol.type")?;
        let symbol_type = match symbol_type.as_str() {
            "capacitor" => Box::new(CapacitorSymbol{ }),
            _ => return Err(ErrorKind::InvalidSymbolType(symbol_type).into()),
        };
        let drawing = symbol_type.draw(config)?;
        Ok(Symbol {
            symbol_type,
            drawing
        })
    }

    pub fn symbol_type(&self) -> &Box<dyn SymbolType> {
        &self.symbol_type
    }

    pub fn drawing(&self) -> &Drawing {
        &self.drawing
    }
}
