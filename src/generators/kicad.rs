use std::env;
use std::fs;

use crate::component::Component;
use crate::drawing::*;
use crate::error::*;
use crate::generators::GeneratorHandler;
use crate::library::Library;

use super::kicad_footprints::KicadFootprints;
use super::kicad_symbols::KicadSymbols;
use super::kicad_symbols_legacy::KicadSymbolsLegacy;

const KICADLIB_DIR: &str = "kicadlib";

pub struct KicadGenerator {}

impl KicadGenerator {
    pub fn new() -> KicadGenerator {
        KicadGenerator {}
    }
}

impl GeneratorHandler for KicadGenerator {
    fn render(&self, name: &str, library: Library) -> Result<()> {
        let config = library.config;
        let unit = config.get_f64("generator.symbol.unit")?;

        let components: Vec<Component> = library
            .components
            .into_iter()
            .map(|mut c| {
                c.symbol = c.symbol.scale(unit, unit);
                c
            })
            .collect();

        // TODO: Remove when obsolete
        info!("rendering legacy KiCad symbol library: '{}.lib'", name);
        fs::create_dir_all(KICADLIB_DIR)?;
        env::set_current_dir(KICADLIB_DIR)?;
        KicadSymbolsLegacy::new(name)
            .settings(&config)
            .render(&components)?;

        info!("rendering KiCad symbol library: '{}.kicad_sym'", name);
        KicadSymbols::new(name)
            .settings(&config)
            .render(&components)?;

        info!("rendering KiCad footprints: '{}.pretty'", name);
        let pattern_dir = format!("{}.pretty", name);
        fs::create_dir_all(&pattern_dir)?;
        env::set_current_dir(&pattern_dir)?;
        KicadFootprints::default()
            .settings(&config)
            .render(&components)?;
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;

        info!("rendering KiCad 3D library: '{}.3dshapes'", name);
        let shapes_dir = format!("{}.3dshapes", name);
        fs::create_dir_all(&shapes_dir)?;
        env::set_current_dir(&shapes_dir)?;
        // TODO: render shapes
        env::set_current_dir(env::current_dir()?.parent().unwrap())?;

        env::set_current_dir(env::current_dir()?.parent().unwrap())?;

        Ok(())
    }
}
