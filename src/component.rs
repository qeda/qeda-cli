use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;
use crate::packages::Packages;
use crate::symbol::Symbol;
use crate::symbols::Symbols;

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub symbol: Symbol,
    pub pattern: Drawing,
    pub model: Drawing,
    pub digest: String,
}

impl Component {
    /// Creates a new `Component` from `Config`.
    pub fn from_config(config: &Config, symbols: &Symbols, packages: &Packages) -> Result<Self> {
        let name = config.get_string("name")?;
        let symbol_handler = symbols.get_handler(&config.get_string("symbol.type")?)?;
        let symbol = symbol_handler.draw(&config)?;
        let package_handler = packages.get_handler(&config.get_string("package.type")?)?;
        let pattern = package_handler.draw_pattern(&config)?;
        let model = package_handler.draw_model(&config)?;
        let digest = config.calc_digest();
        Ok(Component {
            name,
            symbol,
            pattern,
            model,
            digest,
        })
    }

    /// Returns the subset of the conponent's digest (fingerprint).
    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
