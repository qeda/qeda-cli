use std::fs;
use std::path::Path;
use std::str;
use std::time::Duration;

use crate::component::Component;
use crate::config::Config;
use crate::error::*;
use crate::generators::Generators;
use crate::outlines::Outlines;
use crate::packages::Packages;
use crate::symbols::Symbols;

const ID_SEPARATOR: char = '/';
const QEDALIB_DIR: &str = "qedalib";
const YAML_SUFFIX: &str = ".yml";

#[derive(Debug)]
pub struct Library {
    pub config: Config,
    pub symbols: Symbols,
    pub outlines: Outlines,
    pub packages: Packages,
    pub components: Vec<Component>,
}

impl Library {
    /// Creates an empty component library.
    ///
    /// # Examples
    ///
    /// ```
    /// use qeda::library::Library;
    ///
    /// let lib = Library::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: load_config!("qeda.yml"),
            symbols: Symbols::new(),
            outlines: Outlines::new(),
            packages: Packages::new(),
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
    /// let config = Config::from_yaml(yaml).unwrap();
    /// let mut rt = tokio::runtime::Runtime::new().unwrap();
    /// let lib = rt.block_on(Library::from_config(&config)).unwrap();
    ///
    /// assert_eq!(lib.components.len(), 1);
    /// ```
    pub async fn from_config(config: &Config) -> Result<Self> {
        let mut lib = Library::new().merge_config(config);
        let components_hash = config.get_object("components")?;
        let keys = components_hash.keys();
        for key in keys {
            lib.add_component(key.as_str()).await?; // TODO: Replace awaiting in loop by `join_all` of `JoinHandle`s
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
    /// let mut rt = tokio::runtime::Runtime::new().unwrap();
    /// rt.block_on(lib.add_component("capacitor/c0603")).unwrap();
    ///
    /// assert_eq!(lib.components.len(), 1);
    /// ```
    pub async fn add_component(&mut self, id: &str) -> Result<()> {
        let id = id.to_lowercase();

        info!("adding component '{}'", id);
        let component_path = self.local_path(&id);
        let component = if !Path::new(&component_path).exists() {
            self.load_component(&id).await?
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
    /// let mut rt = tokio::runtime::Runtime::new().unwrap();
    /// rt.block_on(lib.load_component("capacitor/c0603")).unwrap();
    /// ```
    pub async fn load_component(&self, id: &str) -> Result<Component> {
        let id = id.to_lowercase();

        info!("loading component '{}'", id);
        let mut url = self.config.get_string("base-url")?;
        if !url.ends_with('/') {
            url += "/";
        }

        if let Some(_manufacturer) = self.manufacturer(&id) {
            // TODO: Get common manufacturer info from README.rst
        }
        url += &self.file_path(&id);
        debug!("URL: {}", url);
        let component_yaml = self
            .get_url_contents(&url)
            .await
            .with_context(|| "component loading failed")?;
        let component = self.parse_component(&id, &component_yaml)?;

        let dir = self.local_dir(&id);
        fs::create_dir_all(&dir)?;
        let component_path = self.local_path(&id);
        debug!("path: {}", component_path);
        fs::write(component_path, component_yaml)?;

        Ok(component)
    }

    /// Generates library for using in EDA.
    ///
    /// Consumes the `Library` by moving to renderer.
    pub fn generate(self, name: &str) -> Result<()> {
        let generator_type = self.config.get_string("generator.type").unwrap();
        let generators = Generators::new();
        generators.get(&generator_type)?.render(name, self)?;
        Ok(())
    }

    // Get contents of page specified by URL
    async fn get_url_contents(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.get_u64("timeout").unwrap()))
            .build()?;

        let response = client.get(url).send().await?.error_for_status()?;
        Ok(response.text().await?)
    }

    // Get file path from specified component ID
    fn file_path(&self, id: &str) -> String {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        path_elems.join("/") + YAML_SUFFIX
    }

    // Get local directory path from specified component ID
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

    // Get local path from specified component ID
    fn local_path(&self, id: &str) -> String {
        QEDALIB_DIR.to_string() + "/" + &self.file_path(&id)
    }

    // Get manufacturer from specified component ID
    fn manufacturer(&self, id: &str) -> Option<String> {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        if path_elems.len() > 1 {
            Some(path_elems[0].to_string())
        } else {
            None
        }
    }

    // Merge the own config with the specified one
    fn merge_config(mut self, config: &Config) -> Self {
        self.config = self.config.merge(config);
        self
    }

    // Parse component's YAML description
    fn parse_component(&self, id: &str, yaml: &str) -> Result<Component> {
        info!("parsing component '{}'", id);
        let config = Config::from_yaml(yaml)?;
        if config.contains("package.outline") {
            dbg!(config.get_str("package.outline")?);
        }
        let component = Component::from_config(&config, &self)?;
        debug!("component short digest: {}", component.digest_short());
        Ok(component)
    }
}

impl Default for Library {
    /// Creates an empty `Library`.
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
