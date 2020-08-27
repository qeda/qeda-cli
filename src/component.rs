use crate::config::Config;
use crate::drawing::Drawing;
use crate::errors::*;
use crate::patterns::Patterns;
use crate::symbols::Symbols;

#[derive(Debug)]
pub struct Component {
    name: String,
    symbol: Drawing,
    pattern: Drawing,
    digest: String,
}

impl Component {
    pub fn from_config(
        config: &Config,
        symbols: &Symbols,
        patterns: &Patterns,
    ) -> Result<Component> {
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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn symbol(&self) -> &Drawing {
        &self.symbol
    }

    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
