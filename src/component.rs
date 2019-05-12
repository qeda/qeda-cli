use yaml_rust::Yaml;

use crate::errors::*;
use crate::symbol::Symbol;
use crate::utils;

pub struct Component {
    name: String,
    symbol: Symbol,
    digest: String,
}

impl Component {
    pub fn from(config: &Yaml) -> Result<Component> {
        let name = utils::get_yaml_string("name", config)?;
        let symbol = Symbol::from(config)?;
        let digest = utils::calc_digest(config);
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
