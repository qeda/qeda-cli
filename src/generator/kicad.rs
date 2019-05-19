use crate::errors::*;
use crate::library::Library;
use crate::generator::GeneratorHandler;
use crate::drawing::Element;

pub struct KicadGenerator {}

impl KicadGenerator {
    pub fn new() -> KicadGenerator {
        KicadGenerator {}
    }
}

impl GeneratorHandler for KicadGenerator {
    fn render(&self, name: &str, library: &Library) -> Result<()> {
        info!("rendering KiCad library: '{}'", name);
        self.render_symbols(name, library)?;
        Ok(())
    }
}

impl KicadGenerator {
    fn render_symbols(&self, name: &str, library: &Library) -> Result<()> {
        let components = library.components();
        for component in components {
            let symbol = component.symbol();
            let elements = symbol.elements();
            for element in elements {
                match element {
                    Element::Line {x0, y0, x1, y1} => {
                        println!("Line: {}, {}, {}, {}", x0, y0, x1, y1);
                    },
                    _ => {},
                }
            }
        }
        Ok(())
    }
}