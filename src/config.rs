use std::fs;
use std::io::Write;
use std::path::Path;

use crypto_hash::{Algorithm, Hasher};
use hex;
use yaml_rust::{yaml, Yaml, YamlLoader, YamlEmitter};

use crate::errors::*;

#[derive(Debug)]
pub struct Config {
    yaml: Yaml,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let yaml_string = fs::read_to_string(path);
        let mut yaml = Yaml::Hash(yaml::Hash::new());

        if let Ok(yaml_string) = yaml_string {
            let docs = YamlLoader::load_from_str(&yaml_string);
            if let Ok(mut docs) = docs {
                if docs.len() > 0 {
                    yaml = docs.pop().unwrap();
                }
            }
        }

        Config {
            yaml,
        }
    }

    pub fn from_file(path: &str) -> Result<Config> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn from_str(yaml: &str) -> Result<Config> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        if docs.len() > 0 {
            Ok(Config {
                yaml: docs.pop().unwrap(),
            })
        } else {
            Err(ErrorKind::InvalidConfig.into())
        }
    }

    pub fn create_if_missing(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::write(path, b"")?;
        }
        Ok(())
    }

    pub fn get_element(&self, key: &str)-> Result<&Yaml> {
        let keys: Vec<&str> = key.split(".").collect();
        let mut element = &self.yaml[keys[0]];
        for key in &keys[1..] {
            element = &element[*key];
        }
        if element.is_badvalue() {
            Err(ErrorKind::MissingElement(key.to_string()).into())
        } else {
            Ok(element)
        }
    }

    pub fn get_hash(&self, key: &str)-> Result<&yaml::Hash> {
        Ok(self.get_element(key)?
            .as_hash()
            .ok_or(ErrorKind::InvalidElementType(key.to_string(), "hash".to_string()))?
        )
    }

    pub fn get_string(&self, key: &str)-> Result<String> {
        Ok(self.get_element(key)?
            .as_str()
            .ok_or(ErrorKind::InvalidElementType(key.to_string(), "string".to_string()))?
            .to_string()
        )
    }

    pub fn get_i64(&self, key: &str)-> Result<i64> {
        Ok(self.get_element(key)?
            .as_i64()
            .ok_or(ErrorKind::InvalidElementType(key.to_string(), "integer".to_string()))?
        )
    }

    pub fn get_u64(&self, key: &str)-> Result<u64> {
        Ok(self.get_element(key)?
            .as_i64()
            .ok_or(ErrorKind::InvalidElementType(key.to_string(), "integer".to_string()))?
            as u64
        )
    }

    pub fn get_f64(&self, key: &str)-> Result<f64> {
        let value = self.get_element(key)?;
        Ok(value.as_f64()
            .unwrap_or(
                value.as_i64()
                .ok_or(ErrorKind::InvalidElementType(key.to_string(), "float".to_string()))?
                as f64
            ))
    }

    pub fn insert_vec_if_missing(&mut self, key: &str) {
        let mut hash = self.yaml_into_hash();
        let key = Yaml::from_str(key);
        if !hash.contains_key(&key) || !hash[&key].is_array() {
            hash.insert(key, Yaml::Array(vec!()));
        }
        self.yaml = Yaml::Hash(hash);
    }

    pub fn insert_hash_if_missing(&mut self, key: &str) {
        let mut hash = self.yaml_into_hash();
        let key = Yaml::from_str(key);
        if !hash.contains_key(&key) || !hash[&key].as_hash().is_some() {
            hash.insert(key, Yaml::Hash(yaml::Hash::new()));
        }
        self.yaml = Yaml::Hash(hash);
    }

    pub fn push_string_to_vec(&mut self, key: &str, value: &str) {
        self.insert_vec_if_missing(key);
        let key = Yaml::from_str(key);
        let mut hash = self.yaml_into_hash();
        let mut vec = hash.remove(&key).unwrap().into_vec().unwrap();
        vec.push(Yaml::from_str(value));
        hash.insert(key, Yaml::Array(vec));
        self.yaml = Yaml::Hash(hash);
    }

    pub fn insert_hash_to_hash(&mut self, key: &str, value: &str) {
        self.insert_hash_if_missing(key);
        let key = Yaml::from_str(key);
        let mut hash = self.yaml_into_hash();
        let mut child_hash = hash.remove(&key).unwrap().into_hash().unwrap();
        child_hash.insert(Yaml::from_str(value), Yaml::Hash(yaml::Hash::new()));
        hash.insert(key, Yaml::Hash(child_hash));
        self.yaml = Yaml::Hash(hash);
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        emitter.dump(&self.yaml)?;
        fs::write(path, yaml_string.as_bytes())?;
        Ok(())
    }

    pub fn calc_digest(&self) -> String {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        self.update_digest(&self.yaml, &mut hasher);
        hex::encode(hasher.finish())
    }

    pub fn merge_with(&mut self, _from: &Config) {

    }
}

// Private methods
impl Config {
    fn yaml_into_hash(&self) -> yaml::Hash {
        // Need to clone as there is no Yaml::as_hash_mut()
        self.yaml.clone().into_hash().unwrap_or(yaml::Hash::new())
    }

    fn update_digest(&self, element: &Yaml, hasher: &mut Hasher) {
        match element {
            Yaml::Real(s) | Yaml::String(s) => hasher.write_all(s.as_bytes()).unwrap(),
            Yaml::Integer(i) => hasher.write_all(&i.to_le_bytes()).unwrap(),
            Yaml::Boolean(b) => {
                let b = *b as u8;
                hasher.write_all(&b.to_le_bytes()).unwrap()
            },
            Yaml::Array(a) => {
                for e in a {
                    self.update_digest(e, hasher);
                }
            },
            Yaml::Hash(h)=> {
                let keys = h.keys();
                for key in keys {
                    hasher.write_all(key.as_str().unwrap().as_bytes()).unwrap();
                    self.update_digest(h.get(key).unwrap(), hasher);
                }
            },
            Yaml::Alias(u) => hasher.write_all(&u.to_le_bytes()).unwrap(),
            _ => (),
        }
    }
}
