#![allow(deprecated)] // because of `Error::description` deprecation in `error_chain`

use error_chain::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Svg(svgdom::ParserError);
        YamlRead(yaml_rust::ScanError);
        YamlWrite(yaml_rust::EmitError);
    }

    errors {
        InvalidConfig {
            display("invalid config")
        }

        InvalidConfigFile(name: String) {
            display("invalid config file: '{}'", name)
        }

        MissingConfigFile(name: String) {
            display("missing config file: '{}'", name)
        }

        MissingElement(name: String) {
            display("missing element '{}' in config", name)
        }

        InvalidElementType(name: String, r#type: String) {
            display("type of config element '{}' is expected to be of type '{}'", name, r#type)
        }

        InvalidSymbolType(r#type: String) {
            display("invalid symbol type: '{}'", r#type)
        }

        InvalidPatternType(r#type: String) {
            display("invalid pattern type: '{}'", r#type)
        }

        InvalidGeneratorType(r#type: String) {
            display("invalid generator type: '{}'", r#type)
        }

        InvalidSvgPath {
            display("invalid SVG path")
        }
    }
}
