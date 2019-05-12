use error_chain::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Reqwest(reqwest::Error);
        Yaml(yaml_rust::ScanError);
    }

    errors {
        InvalidYaml {
            description("invalid YAML")
        }

        MissingElement(name: String) {
            description("missing element")
            display("missing element '{}' in config", name)
        }

        InvalidElementType(name: String, r#type: String) {
            description("invalid element type")
            display("type of config element '{}' is expected to be of type '{}", name, r#type)
        }

        InvalidSymbolType(r#type: String) {
            description("invalid symbol type")
            display("invalid symbol type: '{}'", r#type)
        }
    }
}