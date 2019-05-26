mod chip;

use std::collections::HashMap;
use std::fmt::{self, Debug};

use crate::errors::*;
use crate::config::Config;
use crate::drawing::Drawing;

use chip::ChipPattern;

pub trait PatternHandler {
    fn draw(&self, config: &Config) -> Result<Drawing>;
}

impl Debug for PatternHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PatternHandler")
    }
}

#[derive(Debug)]
pub struct Patterns<'a> {
    handlers: HashMap<&'a str, Box<dyn PatternHandler>>,
}

impl<'a> Patterns<'a>  {
    pub fn new() -> Patterns<'a> {
        let mut handlers: HashMap<&'a str, Box<dyn PatternHandler>> = HashMap::new();
        handlers.insert("chip", Box::new(ChipPattern::new()));

        Patterns {
            handlers,
        }
    }

    pub fn get(&self, key: &str) -> Result<&Box<dyn PatternHandler>> {
        self.handlers.get(key).ok_or(ErrorKind::InvalidPatternHandler(key.to_string()).into())
    }
}