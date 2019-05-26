use crate::errors::*;
use crate::config::Config;
use crate::drawing::Drawing;
use crate::symbols::Symbols;
use crate::patterns::Patterns;

#[derive(Debug)]
pub struct Component {
    name: String,
    symbol: Drawing,
    pattern: Drawing,
    digest: String,
}

impl Component {
    pub fn from_config(config: &Config, symbols: &Symbols, patterns: &Patterns) -> Result<Component> {
        let name = config.get_string("name")?;
        let symbol_handler = symbols.get(&config.get_string("symbol.handler")?)?;
        let symbol = symbol_handler.draw(&config)?;
        let pattern_handler = patterns.get(&config.get_string("pattern.handler")?)?;
        let pattern = pattern_handler.draw(&config)?;
        //let pattern = Pattern::from_config(&config)?;
        let digest = config.calc_digest();
        Ok(Component {
            name,
            symbol,
            pattern,
            digest
        })
    }

    pub fn symbol(&self) -> &Drawing {
        &self.symbol
    }

    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
