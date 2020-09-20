mod kicad;
mod kicad_footprints;
mod kicad_symbols;

use std::collections::HashMap;

use crate::error::*;
use crate::library::Library;
use kicad::KicadGenerator;

pub trait GeneratorHandler {
    fn render(&self, name: &str, library: Library) -> Result<()>;
}

pub struct Generators<'a> {
    handlers: HashMap<&'a str, Box<dyn GeneratorHandler>>,
}

impl<'a> Generators<'a> {
    pub fn new() -> Generators<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn GeneratorHandler>> = HashMap::new();
        handlers.insert("kicad", Box::new(KicadGenerator::new()));

        Generators { handlers }
    }

    pub fn get(&self, key: &str) -> Result<&dyn GeneratorHandler> {
        self.handlers
            .get(key)
            .map(|v| v.as_ref())
            .ok_or_else(|| QedaError::InvalidGeneratorType(key.to_string()).into())
    }
}
