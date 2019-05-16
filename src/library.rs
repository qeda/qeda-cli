use std::fs;
use std::path::Path;
use std::str;
use std::time::Duration;

use crate::errors::*;
use crate::component::Component;
use crate::config::Config;

const ID_SEPARATOR: &str = "/";
const QEDALIB_DIR: &str = "qedalib";
const YAML_SUFFIX: &str = ".yml";

pub struct Library {
    config: Config,
}

impl Library {
    /// Creates an epmty component library.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let lib = Library::new();
    /// ```
    pub fn new() -> Library {
        Library {
            config: load_config!("qeda.yml"),
        }
    }

    pub fn from(config: &Config) -> Result<Library> {
        let mut result = Library::new();
        result.merge_congig_with(config);
        Ok(result)
    }

    /// Adds component to library config.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let lib = Library::new();
    /// lib.add_component("capacitor:c0603")?;
    /// ```
    pub fn add_component(&self, id: &str) -> Result<()> {
        let id = id.to_lowercase();

        info!("adding component '{}'", id);
        let component_path = self.local_path(&id);
        if !Path::new(&component_path).exists() {
            self.load_component(&id)?;
        } else {
            let component_yaml = fs::read_to_string(component_path)?;
            self.parse_component(&id, &component_yaml)?;
        }
        Ok(())
    }

    /// Loads component config from remote repository.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let lib = Library::new();
    /// lib.load_component("capacitor:c0603")?;
    /// ```
    pub fn load_component(&self, id: &str) -> Result<()> {
        let id = id.to_lowercase();

        info!("loading component '{}'", id);
        let mut url = self.config.get_string("base_url")?;
        if !url.ends_with("/") {
            url += "/";
        }
        
        if let Some(_manufacturer) = self.manufacturer(&id) {
            // TODO: Get common manufacturer info from README.rst
        }
        url += &self.file_path(&id);
        debug!("URL: {}", url);
        let component_yaml = self.get_url_contents(&url).chain_err(|| "component loading failed")?;
        self.parse_component(&id, &component_yaml)?;

        let dir = self.local_dir(&id);
        fs::create_dir_all(&dir)?;
        let component_path = self.local_path(&id);
        debug!("path: {}", component_path);
        fs::write(component_path, component_yaml)?;
                
        Ok(())
    }
}

// Private methods
impl Library {
    fn get_url_contents(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            //.timeout(Duration::from_secs(self.config.get_u64("timeout_secs")?["timeout_secs"].as_i64().unwrap() as u64))
            .timeout(Duration::from_secs(self.config.get_u64("timeout_secs")?))
            .build()?;
        
        let mut response = client.get(url).send()?.error_for_status()?;
        Ok(response.text()?)
    }

    fn file_path(&self, id: &str) -> String {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        path_elems.join("/") + YAML_SUFFIX
    }

    fn local_dir(&self, id: &str) -> String {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        let last_but_one = path_elems.len()-1;
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

    fn parse_component(&self, id: &str, yaml: &str) -> Result<()> {
        info!("parsing component '{}'", id);
        let config = Config::from_str(yaml)?;
        let component = Component::from(&config)?; // Validate config
        debug!("component short digest: {}", component.digest_short());
        Ok(())
    }

    fn manufacturer(&self, id: &str) -> Option<String> {
        let path_elems: Vec<&str> = id.split(ID_SEPARATOR).collect();
        if path_elems.len() > 1 {
            Some(path_elems[0].to_string())
        } else {
            None
        }
    }

    fn merge_congig_with(&mut self, config: &Config) {
        self.config.merge_with(config);
    }
}