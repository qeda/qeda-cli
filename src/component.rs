use yaml_rust::Yaml;

use crate::errors::*;
use crate::schematic::Schematic;
use crate::utils;

pub struct Component {
    name: String,
    schematic: Schematic,
    digest: String,
}

impl Component {
    pub fn from(config: &Yaml) -> Result<Component> {
        let name = utils::get_yaml_string("name", config)?;
        let schematic = Schematic::from(config)?;
        let digest = utils::calc_digest(config);
        Ok(Component {
            name,
            schematic,
            digest
        })
    }

    pub fn digest_short(&self) -> &str {
        &self.digest[0..12]
    }
}
