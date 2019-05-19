mod chip;

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
pub struct Pattern {
    handler: Box<dyn PatternHandler>,
    drawing: Drawing,
}

impl Pattern {
    pub fn from_config(config: &Config) -> Result<Pattern> {
        let pattern_handler = config.get_string("pattern.handler")?;
        let handler = match pattern_handler.as_str() {
            "chip" => Box::new(ChipPattern {}),
            _ => return Err(ErrorKind::InvalidPatternHandler(pattern_handler).into()),
        };
        let drawing = handler.draw(config)?;
        Ok(Pattern {
            handler,
            drawing
        })
    }

    pub fn handler(&self) -> &Box<dyn PatternHandler> {
        &self.handler
    }

    pub fn drawing(&self) -> &Drawing {
        &self.drawing
    }
}