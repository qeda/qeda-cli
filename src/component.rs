use crate::errors::*;
use crate::config::Config;
use crate::symbol::Symbol;

pub struct Component {
    name: String,
    symbol: Symbol,
    digest: String,
}

impl Component {
    pub fn from(config: &Config) -> Result<Component> {
        let name = config.get_string("name")?;
        let symbol = Symbol::from(&config)?;
        let digest = config.calc_digest();
        Ok(Component {
            name,
            symbol,
            digest
        })
    }

    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
