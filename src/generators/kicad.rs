use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::errors::*;
use crate::library::Library;
use crate::generators::GeneratorHandler;
use crate::geometry::Transform;
use crate::drawing::Element;

const KICADLIB_DIR: &str = "kicadlib";

pub struct KicadGenerator {}

impl KicadGenerator {
    pub fn new() -> KicadGenerator {
        KicadGenerator {}
    }
}

impl GeneratorHandler for KicadGenerator {
    fn render(&self, name: &str, library: &Library) -> Result<()> {
        info!("rendering KiCad symbol library: '{}.lib'", name);
        fs::create_dir_all( KICADLIB_DIR)?;
        self.render_symbols(name, library)?;

        info!("rendering KiCad pattern library: '{}.pretty'", name);
        fs::create_dir_all(format!("{}/{}.pretty", KICADLIB_DIR, name))?;

        info!("rendering KiCad 3D library: '{}.3dshapes'", name);
        fs::create_dir_all(format!("{}/{}.3dshapes", KICADLIB_DIR, name))?;
        Ok(())
    }
}

impl KicadGenerator {
    fn render_symbols(&self, name: &str, library: &Library) -> Result<()> {
        let grid = library.config().get_f64("generator.symbol_grid")?;
        let mut f = File::create(format!("{}/{}.lib", KICADLIB_DIR, name))?;
        f.write(b"EESchema-LIBRARY Version 2.4\n")?;
        f.write(b"#encoding utf-8\n")?;
        let components = library.components();
        for component in components {
            write!(f, "#\n# {}\n#\n", component.name())?;
            let symbol = component.symbol();
            write!(f, "DEF {} {} 0 {} {} {} {} L {}\n",
                component.name(),
                symbol.attr("ref_des", "U"),
                5, // Space. TODO: Replace by attribute
                symbol.attr("show_pin_numbers", "N"),
                symbol.attr("show_pin_names", "N"),
                1, // Symbols count. TODO: Replace by attribute
                symbol.attr("power", "N"),
            )?;
            let elements = symbol.elements();
            f.write(b"DRAW\n")?;
            for (i, element) in elements.iter().enumerate() {
                match element {
                    Element::Line(l) => {
                        let mut l = l.clone();
                        l.scale(grid, grid);
                        write!(f, "P 2 {} 1 {} {} {} {} {} N\n",
                            i,
                            l.width.round(),
                            l.p.0.x.round(), l.p.0.y.round(),
                            l.p.1.x.round(), l.p.1.y.round()
                        )?;
                        println!("Line: {}, {}, {}, {}", l.p.0.x, l.p.0.y, l.p.1.x, l.p.1.y);
                    },
                    _ => (),
                }
            }
            f.write(b"ENDDRAW\n")?;
            f.write(b"ENDDEF\n")?;
        }
        f.write(b"#\n#End Library\n")?;
        Ok(())
    }
}
