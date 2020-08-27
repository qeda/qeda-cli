mod chip;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::config::Config;
use crate::drawing::Drawing;
use crate::error::*;

use chip::ChipPattern;

pub trait PatternHandler {
    fn draw(&self, config: &Config) -> Result<Drawing>;
}

impl Debug for dyn PatternHandler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PatternHandler")
    }
}

#[derive(Debug)]
pub struct Patterns<'a> {
    handlers: HashMap<&'a str, Box<dyn PatternHandler>>,
}

impl<'a> Patterns<'a> {
    pub fn new() -> Patterns<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn PatternHandler>> = HashMap::new();
        handlers.insert("chip", Box::new(ChipPattern::new()));

        Patterns { handlers }
    }

    pub fn get_handler(&self, key: &str) -> Result<&Box<dyn PatternHandler>> {
        self.handlers
            .get(key)
            .ok_or(QedaError::InvalidPatternType(key.to_string()).into())
    }
}
