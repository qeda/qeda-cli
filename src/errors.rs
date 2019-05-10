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
    }
}