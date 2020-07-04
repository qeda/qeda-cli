use std::fs;
use std::path::Path;
use std::str;
use std::time::Duration;

use crate::errors::*;
use crate::config::Config;
use crate::symbols::Symbols;
use crate::patterns::Patterns;
use crate::component::Component;
use crate::generators::Generators;

const ID_SEPARATOR: &str = "/";
const QEDALIB_DIR: &str = "qedalib";
const YAML_SUFFIX: &str = ".yml";

#[derive(Debug)]
pub struct Library<'a> {
    config: Config,
    symbols: Symbols<'a>,
    patterns: Patterns<'a>,
    components: Vec<Component>,
}

impl<'a> Library<'a> {
    /// Creates an empty component library.
    ///
    /// # Examples
    ///
    /// ```
    /// use qeda::library::Library;
    ///
    /// let lib = Library::new();
    /// ```
    pub fn new() -> Library<'a> {
        Library {
            config: load_config!("qeda.yml"),
            symbols: Symbols::new(),
            patterns: Patterns::new(),
            components: Vec::new(),
        }
    }

    // Creates a component library from config.
    ///
    /// # Examples
    ///
    /// ```
    /// use qeda::config::Config;
    /// use qeda::library::Library;
    ///
    /// let yaml = "
    /// components:
    ///   capacitor/c0603: {}
    /// ";
    /// let config = Config::from_str(yaml).unwrap();
    /// let lib = Library::from_config(&config).unwrap();
    ///
    /// assert_eq!(lib.components().len(), 1);
    /// ```
    pub fn from_config(config: &Config) -> Result<Library> {
        let mut lib = Library::new();
        lib.merge_config_with(config);
        let components_hash = config.get_hash("components")?;
        let keys = components_hash.keys();
        for key in keys {
            lib.add_component(key.as_str().unwrap())?;
        }
        Ok(lib)
    }

    /// Adds component to library config.
    ///
    /// # Examples
    ///
    /// ```
    /// use qeda::library::Library;
    ///
    /// let mut lib = Library::new();
    /// lib.add_component("capacitor/c0603").unwrap();
    ///
    /// assert_eq!(lib.components().len(), 1);
    /// ```
    pub fn add_component(&mut self, id: &str) -> Result<()> {
        let id = id.to_lowercase();

        info!("adding component '{}'", id);
        let component_path = self.local_path(&id);
        let component = if !Path::new(&component_path).exists() {
            self.load_component(&id)?
        } else {
            let component_yaml = fs::read_to_string(component_path)?;
            self.parse_component(&id, &component_yaml)?
        };
        self.components.push(component);
        Ok(())
    }

    /// Loads component config from remote repository.
    ///
    /// # Examples
    ///
    /// ```
    /// use qeda::library::Library;
    ///
    /// let lib = Library::new();
    /// lib.load_component("capacitor/c0603").unwrap();
    /// ```
    pub fn load_component(&self, id: &str) -> Result<Component> {
        let id = id.to_lowercase();

        info!("loading component '{}'", id);
        let mut url = self.config.get_string("base-url")?;
        if !url.ends_with("/") {
            url += "/";
        }

        if let Some(_manufacturer) = self.manufacturer(&id) {
            // TODO: Get common manufacturer info from README.rst
        }
        url += &self.file_path(&id);
        debug!("URL: {}", url);
        let component_yaml = self.get_url_contents(&url).chain_err(|| "component loading failed")?;
        let component = self.parse_component(&id, &component_yaml)?;

        let dir = self.local_dir(&id);
        fs::create_dir_all(&dir)?;
        let component_path = self.local_path(&id);
        debug!("path: {}", component_path);
        fs::write(component_path, component_yaml)?;

        Ok(component)
    }

    /// Returns components array.
    pub fn components(&self) -> &Vec<Component> {
        &self.components
    }

    /// Returns library config.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Generates library for using in EDA.
    pub fn generate(&self, name: &str) -> Result<()> {
        let generator_type = self.config.get_string("generator.type")?;
        let generators = Generators::new();
        generators.get(&generator_type)?.render(name, self)?;
        Ok(())
    }
}

// Private methods
impl<'a> Library<'a> {
    fn get_url_contents(&self, url: &str) -> Result<String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(self.config.get_u64("timeout")?))
            .build()?;

        let response = client.get(url).send()?.error_for_status()?;
        Ok(response.text()?)
    }

    fn file_path(&self, id: &str) -> String {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        path_elems.join("/") + YAML_SUFFIX
    }

    fn local_dir(&self, id: &str) -> String {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        let last_but_one = path_elems.len() - 1;
        let result = QEDALIB_DIR.to_string();
        if last_but_one > 0 {
            result + "/" + &path_elems[..last_but_one].join("/")
        } else {
            result
        }
    }

    fn local_path(&self, id: &str) -> String {
        QEDALIB_DIR.to_string() + "/" + &self.file_path(&id)
    }

    fn manufacturer(&self, id: &str) -> Option<String> {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        if path_elems.len() > 1 {
            Some(path_elems[0].to_string())
        } else {
            None
        }
    }

    fn merge_config_with(&mut self, config: &Config) {
        self.config.merge_with(config);
    }

    fn parse_component(&self, id: &str, yaml: &str) -> Result<Component> {
        info!("parsing component '{}'", id);
        let config = Config::from_str(yaml)?;
        let component = Component::from_config(&config, &self.symbols, &self.patterns)?;
        debug!("component short digest: {}", component.digest_short());
        Ok(component)
    }
}
