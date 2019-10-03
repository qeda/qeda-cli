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
            description("invalid config")
        }

        InvalidConfigFile(name: String) {
            description("invalid config file")
            display("invalid config file: '{}'", name)
        }

        MissingConfigFile(name: String) {
            description("missing config file")
            display("missing config file: '{}'", name)
        }

        MissingElement(name: String) {
            description("missing element")
            display("missing element '{}' in config", name)
        }

        InvalidElementType(name: String, r#type: String) {
            description("invalid element type")
            display("type of config element '{}' is expected to be of type '{}'", name, r#type)
        }

        InvalidSymbolType(r#type: String) {
            description("invalid symbol type")
            display("invalid symbol type: '{}'", r#type)
        }

        InvalidPatternType(r#type: String) {
            description("invalid pattern type")
            display("invalid pattern type: '{}'", r#type)
        }

        InvalidGeneratorType(r#type: String) {
            description("invalid generator type")
            display("invalid generator type: '{}'", r#type)
        }

        InvalidSvgPath {
            description("invalid SVG path")
        }
    }
}