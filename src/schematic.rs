use yaml_rust::Yaml;

use crate::errors::*;
use crate::drawing::Drawing;
use crate::utils;

use crate::symbols:: {
    capacitor::CapacitorSymbol
};

pub trait SchematicElement {
}

pub trait Symbol {
    fn draw(&self, config: &Yaml) -> Result<Drawing>;
}

pub struct Schematic {
    symbol: Box<dyn Symbol>,
    drawing: Drawing,
}

impl Schematic {
    pub fn from(config: &Yaml) -> Result<Schematic>{
        let symbol_type = utils::get_yaml_string("symbol.type", config)?;
        let symbol = match symbol_type.as_str() {
            "capacitor" => Box::new(CapacitorSymbol{ }),
            _ => return Err(ErrorKind::InvalidSymbolType(symbol_type).into()),
        };
        let drawing = symbol.draw(config)?;
        Ok(Schematic {
            symbol,
            drawing
        })
    }

    pub fn symbol(&self) -> &Box<dyn Symbol> {
        &self.symbol
    }

    pub fn drawing(&self) -> &Drawing {
        &self.drawing
    }
}
