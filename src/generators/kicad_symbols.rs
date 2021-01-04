use chrono::Local;
use std::fs::File;
use std::io::prelude::*;

use crate::component::Component;
use crate::config::Config;
use crate::error::*;

#[derive(Debug, Default)]
pub struct KicadSymbols {
    name: String,
    font_size_name: i64,
    font_size_pin: i64,
    font_size_ref_des: f64,
    font_size_value: f64,
    space_pin: i64,
}

impl KicadSymbols {
    pub fn new(name: &str) -> Self {
        KicadSymbols {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Renders symbols to a KiCad symbol library.
    pub fn render(self, components: &[Component]) -> Result<()> {
        let mut f = File::create(format!("{}.kicad_sym", self.name))?;
        let today = Local::now().date();
        writeln!(
            f,
            "(kicad_symbol_lib (version {}) (generator qeda)",
            today.format("%Y%m%d")
        )?;

        for component in components {
            let name = &component.name;
            let symbol = &component.symbol;
            ensure!(
                !symbol.parts.is_empty(),
                QedaError::InvalidSymbolNoParts(name.to_string())
            );
            info!("  • symbol: '{}'", name);
            writeln!(f, "  (symbol \"{}:{}\"", self.name, component.name)?;
            writeln!(f, "  )")?;
        }
        writeln!(f, ")")?;
        Ok(())
    }

    /// Builds an `KicadSymbols` with applied settings from `Config`.
    pub fn settings(mut self, lib_cfg: &Config) -> Self {
        let unit = lib_cfg.get_f64("generator.symbol.unit").unwrap();

        self.font_size_name =
            (unit * lib_cfg.get_f64("symbol.font-size.name").unwrap()).round() as i64;
        self.font_size_pin =
            (unit * lib_cfg.get_f64("symbol.font-size.pin").unwrap()).round() as i64;
        self.font_size_ref_des =
            (unit * lib_cfg.get_f64("symbol.font-size.ref-des").unwrap()).round();
        self.font_size_value = (unit * lib_cfg.get_f64("symbol.font-size.value").unwrap()).round();
        self.space_pin = (unit * lib_cfg.get_f64("symbol.space.pin").unwrap()).round() as i64;

        self
    }
}
