use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;
use crate::patterns::Patterns;
use crate::symbols::Symbols;

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub symbol: Drawing,
    pub pattern: Drawing,
    pub digest: String,
}

impl Component {
    /// Creates a new `Component` from `Config`.
    pub fn from_config(config: &Config, symbols: &Symbols, patterns: &Patterns) -> Result<Self> {
        let name = config.get_string("name")?;
        let symbol_handler = symbols.get_handler(&config.get_string("symbol.type")?)?;
        let symbol = symbol_handler.draw(&config)?;
        let pattern_handler = patterns.get_handler(&config.get_string("pattern.type")?)?;
        let pattern = pattern_handler.draw(&config)?;
        let digest = config.calc_digest();
        Ok(Component {
            name,
            symbol,
            pattern,
            digest,
        })
    }

    /// Returns the subset of the conponent's digest (fingerprint).
    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
