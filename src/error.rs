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

    #[error("invalid package type: '{0}'")]
    InvalidPackageType(String),

    #[error(
        "Invalid pin count, it should be the same at the both sides: 'count({0})' != 'count({1})'"
    )]
    InvalidPinCount(String, String),

    #[error("invalid pin name: '{0}'")]
    InvalidPinName(String),

    #[error("invalid pin range '{0}', name base should be the same: '{1}' != '{2}'")]
    InvalidPinRangeNameBase(String, String, String),

    #[error("invalid pin number: '{0}'")]
    InvalidPinNumber(String),

    #[error("invalid SVG path")]
    InvalidSvgPath,

    #[error("invalid SVG pin ID: '{0}'")]
    InvalidSvgPinId(String),

    #[error("invalid SVG pin name: '{0}'")]
    InvalidSvgPinName(String),

    #[error("invalid symbol, no parts: '{0}'")]
    InvalidSymbolNoParts(String),

    #[error("invalid symbol type: '{0}'")]
    InvalidSymbolType(String),

    #[error("missing config file: '{0}'")]
    MissingConfigFile(String),

    #[error("missing element '{0}' in config")]
    MissingElement(String),

    #[error("unsupported SVG units: '{0}'")]
    UnsupportedSvgUnits(String),
}
