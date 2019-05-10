use yaml_rust::{yaml};

use crate::errors::*;

pub struct Component {

}

impl Component {
    pub fn validate(_config: &yaml::Hash) -> Result<()> {
        // TODO: Implement config valifation
        Ok(())
    }
}