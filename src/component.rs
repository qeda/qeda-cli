use crate::errors::*;
use crate::config::Config;
use crate::drawing::Drawing;
use crate::symbol::Symbol;
use crate::pattern::Pattern;

#[derive(Debug)]
pub struct Component {
    name: String,
    symbol: Drawing,
    pattern: Pattern,
    digest: String,
}

impl Component {
    pub fn from_config(config: &Config) -> Result<Component> {
        let name = config.get_string("name")?;
        let symbol = Symbol::new().draw(&config)?;
        let pattern = Pattern::from_config(&config)?;
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
