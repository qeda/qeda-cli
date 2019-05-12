use std::{fs, str, time::Duration};
use yaml_rust::YamlLoader;

use crate::errors::*;
use crate::component::Component;

const PATH_SEPARATOR: &str = ":";
const QEDALIB_DIR: &str = "qedalib";
const YAML_SUFFIX: &str = ".yml";

struct LibraryConfig {
    base_url: &'static str,
    timeout_secs: u64
}

pub struct Library {
    config: LibraryConfig
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
            config: LibraryConfig {
                base_url: "https://raw.githubusercontent.com/qeda/lib/master/",
                timeout_secs: 5
            }
        }
    }

    /// Adds component to library config.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let lib = Library::new();
    /// lib.add_component("capacitor:c0603")?;
    /// ```
    pub fn add_component(&self, _path: &str) -> Result<()> {
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
    pub fn load_component(&self, path: &str) -> Result<()> {
        let path = path.to_lowercase();

        info!("loading component '{}'", path);
        let mut url = self.config.base_url.to_string();
        if !url.ends_with("/") {
            url += "/";
        }
        let path_elems: Vec<&str> = path.split(PATH_SEPARATOR).collect();
        let component_path = if path_elems.len() > 1 {
            path_elems[1..].join("/")
        } else {
            path_elems[0].to_string()
        };
        
        if path_elems.len() > 1 {
            let manufacturer = path_elems[0];
            // TODO: Get common manufacturer info from README.rst
            url += manufacturer;
            url += "/";
        }
        url = url + &component_path + YAML_SUFFIX;
        debug!("URL: {}", url);
        let component_yaml = self.get_url_contents(&url).chain_err(|| "component loading failed")?;

        info!{"parsing component '{}'", path}
        let component_config = &YamlLoader::load_from_str(&component_yaml)?[0];
        let component = Component::from(component_config)?; // Validate config
        debug!("component short digest: {}", component.digest_short());

        let dir: String = QEDALIB_DIR.to_string() + "/" + &path_elems[..path_elems.len()-1].join("/");
        let component_filename = path_elems.last().unwrap().to_string() + YAML_SUFFIX;
        fs::create_dir_all(&dir)?;
        let component_path = dir + "/" + &component_filename;
        debug!("path: {}", component_path);
        fs::write(component_path, component_yaml)?;
                
        Ok(())
    }
}

// Private methods
impl Library {
    fn get_url_contents(&self, url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(self.config.timeout_secs))
            .build()?;
        
        let mut response = client.get(url).send()?.error_for_status()?;
        Ok(response.text()?)
    }
}