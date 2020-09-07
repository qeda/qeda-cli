use std::fs;
use std::io::Write;
use std::path::Path;

use crypto_hash::{Algorithm, Hasher};
use hex;
use regex::Regex;
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

use crate::error::*;

#[derive(Debug)]
pub struct Config {
    yaml: Yaml,
}

impl Config {
    pub fn new() -> Config {
        Config {
            yaml: Yaml::Hash(yaml::Hash::new()),
        }
    }

    pub fn from_file(path: &str) -> Result<Config> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn from_str(yaml: &str) -> Result<Config> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(docs.len() > 0, QedaError::InvalidConfig);
        Ok(Config {
            yaml: docs.pop().unwrap(),
        })
    }

    pub fn create_if_missing(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::write(path, b"---")?;
        }
        Ok(())
    }

    pub fn get_element(&self, key: &str) -> Result<&Yaml> {
        let keys: Vec<&str> = key.split(".").collect();
        let mut element = &self.yaml[keys[0]];
        for key in &keys[1..] {
            element = &element[*key];
        }
        if element.is_badvalue() {
            Err(QedaError::MissingElement(key.to_string()).into())
        } else {
            Ok(element)
        }
    }

    pub fn get_hash(&self, key: &str) -> Result<&yaml::Hash> {
        Ok(self
            .get_element(key)?
            .as_hash()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "hash"))?)
    }

    pub fn get_range(&self, key: &str) -> Result<(f64, f64)> {
        let elem = self.get_element(key)?;
        match elem {
            Yaml::Integer(i) => {
                let f = *i as f64;
                Ok((f, f))
            }
            Yaml::Real(_) => {
                let f = elem
                    .as_f64()
                    .ok_or(QedaError::InvalidElementType(key.to_string(), "range: f64"))?;
                Ok((f, f))
            }
            Yaml::String(s) => {
                let re = Regex::new(r"(\d+\.*\d*)\s*(\.\.|\+/-)\s*(\d+\.*\d*)").unwrap();
                let caps = re.captures(s).ok_or(QedaError::InvalidElementType(
                    key.to_string(),
                    "range: f64..f64 or f64 +/- f64",
                ))?;
                let f1 = caps[1].parse::<f64>()?;
                let f2 = caps[3].parse::<f64>()?;
                match &caps[2] {
                    ".." => Ok((f1, f2)),
                    "+/-" => Ok((f1 - f2, f1 + f2)),
                    _ => Err(QedaError::InvalidElementType(
                        key.to_string(),
                        "range: f64..f64 or f64 +/- f64",
                    )
                    .into()),
                }
            }
            Yaml::Array(a) if a.len() == 2 => {
                let f1 = a[0]
                    .as_f64()
                    .or(a[0].as_i64().and_then(|v| Some(v as f64)))
                    .ok_or(QedaError::InvalidElementType(
                        key.to_string(),
                        "range: [f64, f64]",
                    ))?;
                let f2 = a[1]
                    .as_f64()
                    .or(a[1].as_i64().and_then(|v| Some(v as f64)))
                    .ok_or(QedaError::InvalidElementType(
                        key.to_string(),
                        "range: [f64, f64]",
                    ))?;
                Ok((f1, f2))
            }
            _ => Err(QedaError::InvalidElementType(key.to_string(), "range").into()),
        }
    }

    pub fn get_str(&self, key: &str) -> Result<&str> {
        Ok(self
            .get_element(key)?
            .as_str()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "str"))?)
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        Ok(self
            .get_element(key)?
            .as_str()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "string"))?
            .to_string())
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        Ok(self
            .get_element(key)?
            .as_i64()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "integer"))?)
    }

    pub fn get_u64(&self, key: &str) -> Result<u64> {
        Ok(self
            .get_element(key)?
            .as_i64()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "integer"))? as u64)
    }

    pub fn get_f64(&self, key: &str) -> Result<f64> {
        let value = self.get_element(key)?;
        Ok(value
            .as_f64()
            .or(value.as_i64().and_then(|v| Some(v as f64)))
            .ok_or(QedaError::InvalidElementType(key.to_string(), "float"))?)
    }

    pub fn insert_vec_if_missing(self, key: &str) -> Self {
        let mut hash = self.yaml_into_hash();
        let key = Yaml::from_str(key);
        if !hash.contains_key(&key) || !hash[&key].is_array() {
            hash.insert(key, Yaml::Array(vec![]));
        }
        Config {
            yaml: Yaml::Hash(hash),
        }
    }

    pub fn insert_hash_if_missing(self, key: &str) -> Self {
        let mut hash = self.yaml_into_hash();
        let key = Yaml::from_str(key);
        if !hash.contains_key(&key) || !hash[&key].as_hash().is_some() {
            hash.insert(key, Yaml::Hash(yaml::Hash::new()));
        }
        Config {
            yaml: Yaml::Hash(hash),
        }
    }

    pub fn push_string_to_vec(self, key: &str, value: &str) -> Self {
        let yaml_key = Yaml::from_str(key);
        let mut hash = self.insert_vec_if_missing(key).yaml_into_hash();
        let mut vec = hash.remove(&yaml_key).unwrap().into_vec().unwrap();
        vec.push(Yaml::from_str(value));
        hash.insert(yaml_key, Yaml::Array(vec));
        Config {
            yaml: Yaml::Hash(hash),
        }
    }

    pub fn insert_hash_to_hash(self, key: &str, value: &str) -> Self {
        let yaml_key = Yaml::from_str(key);
        let mut hash = self.insert_hash_if_missing(key).yaml_into_hash();
        let mut child_hash = hash.remove(&yaml_key).unwrap().into_hash().unwrap();
        child_hash.insert(Yaml::from_str(value), Yaml::Hash(yaml::Hash::new()));
        hash.insert(yaml_key, Yaml::Hash(child_hash));
        Config {
            yaml: Yaml::Hash(hash),
        }
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

    pub fn merge(self, _from: &Config) -> Self {
        // TODO: Implement
        self
    }
}

// Private methods
impl Config {
    fn yaml_into_hash(self) -> yaml::Hash {
        self.yaml.into_hash().unwrap_or(yaml::Hash::new())
    }

    fn update_digest(&self, element: &Yaml, hasher: &mut Hasher) {
        match element {
            Yaml::Real(s) | Yaml::String(s) => hasher.write_all(s.as_bytes()).unwrap(),
            Yaml::Integer(i) => hasher.write_all(&i.to_le_bytes()).unwrap(),
            Yaml::Boolean(b) => {
                let b = *b as u8;
                hasher.write_all(&b.to_le_bytes()).unwrap()
            }
            Yaml::Array(a) => {
                for e in a {
                    self.update_digest(e, hasher);
                }
            }
            Yaml::Hash(h) => {
                let keys = h.keys();
                for key in keys {
                    hasher.write_all(key.as_str().unwrap().as_bytes()).unwrap();
                    self.update_digest(h.get(key).unwrap(), hasher);
                }
            }
            Yaml::Alias(u) => hasher.write_all(&u.to_le_bytes()).unwrap(),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() -> Result<()> {
        let config = Config::from_str(
            r"
        A: 1
        B: 1.3
        C: 1.0..2.0
        D: 5 .. 6
        E: 5 +/- 1.1
        F: [1, 2]
        G: [2.25, 6.5]
        ",
        )?;

        assert_eq!(config.get_range("A")?, (1.0, 1.0));
        assert_eq!(config.get_range("B")?, (1.3, 1.3));
        assert_eq!(config.get_range("C")?, (1.0, 2.0));
        assert_eq!(config.get_range("D")?, (5.0, 6.0));
        assert_eq!(config.get_range("E")?, (3.9, 6.1));
        assert_eq!(config.get_range("F")?, (1.0, 2.0));
        assert_eq!(config.get_range("G")?, (2.25, 6.5));
        Ok(())
    }
}
