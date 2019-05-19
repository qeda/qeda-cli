mod kicad;

use std::collections::HashMap;

use crate::errors::*;
use crate::library::Library;
use kicad::KicadGenerator;

pub trait GeneratorHandler {
    fn render(&self, name: &str, library: &Library) -> Result<()>;
}

pub struct Generator<'a> {
    handlers: HashMap<&'a str, Box<dyn GeneratorHandler>>,
}

impl<'a> Generator<'a> {
    pub fn new() -> Generator<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn GeneratorHandler>> = HashMap::new();
        handlers.insert("kicad", Box::new(KicadGenerator::new()));

        Generator {
            handlers,
        }
    }

    pub fn handler(&self, key: &str) -> Result<&Box<dyn GeneratorHandler>> {
        self.handlers.get(key).ok_or(ErrorKind::InvalidGeneratorHandler(key.to_string()).into())
    }
}
