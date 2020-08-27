pub use anyhow::{bail, ensure, Context, Error, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QedaError {
    #[error("invalid config")]
    InvalidConfig,

    #[error("type of config element '{0}' is expected to be of type '{1}'")]
    InvalidElementType(String, &'static str),

    #[error("invalid generator type: '{0}'")]
    InvalidGeneratorType(String),

    #[error("invalid pattern type: '{0}'")]
    InvalidPatternType(String),

    #[error("invalid SVG path")]
    InvalidSvgPath,

    #[error("invalid symbol type: '{0}'")]
    InvalidSymbolType(String),

    #[error("missing config file: '{0}'")]
    MissingConfigFile(String),

    #[error("missing element '{0}' in config")]
    MissingElement(String),

    #[error("unsupported SVG units: '{0}'")]
    UnsupportedSvgUnits(String),
}
