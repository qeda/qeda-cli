use std::fs;
use std::io::Write;
use std::path::Path;

use crypto_hash::{Algorithm, Hasher};
use hex;
use regex::Regex;
use serde_json::{Map, Number, Value};
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

use crate::error::*;

#[derive(Debug)]
pub struct Config {
    json: Value,
}

impl Config {
    /// Creates an empty `Config`.
    pub fn new() -> Self {
        Config {
            json: Value::Object(Map::new()),
        }
    }

    /// Creates a new `Config` from JSON string.
    pub fn from_json(json: &str) -> Result<Config> {
        let value: Value = serde_json::from_str(json)?;
        Ok(Config { json: value })
    }

    /// Creates a new `Config` from YAML file.
    pub fn from_yaml_file(path: &str) -> Result<Config> {
        Self::from_yaml(&fs::read_to_string(path)?)
    }

    /// Creates a new `Config` from YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Config> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(docs.len() > 0, QedaError::InvalidConfig);
        let mut json = Self::yaml_to_json(docs.pop().unwrap())?;
        if json == Value::Null {
            json = Value::Object(Map::new());
        }
        Ok(Config { json: json })
    }

    /// Creates a new file if it doesn't exist.
    pub fn create_if_missing(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::write(path, b"---")?;
        }
        Ok(())
    }

    pub fn get_bool(&self, key: &str) -> Result<bool> {
        Ok(self
            .get_element(key)?
            .as_bool()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "bool"))?)
    }

    /// Returns a config element.
    pub fn get_element(&self, key: &str) -> Result<&Value> {
        let keys: Vec<&str> = key.split(".").collect();
        let mut element = &self.json[keys[0]];
        for key in &keys[1..] {
            element = &element[*key];
        }
        if element.is_null() {
            Err(QedaError::MissingElement(key.to_string()).into())
        } else {
            Ok(element)
        }
    }

    pub fn get_i64(&self, key: &str) -> Result<i64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "number"))?
            .round() as i64)
    }

    pub fn get_object(&self, key: &str) -> Result<&Map<String, Value>> {
        Ok(self
            .get_element(key)?
            .as_object()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "object"))?)
    }

    pub fn get_range(&self, key: &str) -> Result<(f64, f64)> {
        let elem = self.get_element(key)?;
        match elem {
            Value::Number(n) => {
                let f = n.as_f64().unwrap();
                Ok((f, f))
            }
            Value::String(s) => {
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
            Value::Array(a) if a.len() == 2 => {
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
            .ok_or(QedaError::InvalidElementType(key.to_string(), "string"))?)
    }

    pub fn get_string(&self, key: &str) -> Result<String> {
        Ok(self
            .get_element(key)?
            .as_str()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "string"))?
            .to_string())
    }

    pub fn get_u64(&self, key: &str) -> Result<u64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "number"))?
            .round() as u64)
    }

    pub fn get_f64(&self, key: &str) -> Result<f64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "number"))?)
    }

    pub fn insert_object(&mut self, key: &str, value: &str) -> Result<()> {
        let map = self.json.as_object_mut().unwrap();
        if !map.contains_key(key) {
            // Insert child if doesn't exist
            map.insert(key.to_string(), Value::Object(Map::new()));
        }
        let child = map[key]
            .as_object_mut()
            .ok_or(QedaError::InvalidElementType(key.to_string(), "object"))?;
        child.insert(value.to_string(), Value::Object(Map::new()));
        Ok(())
    }

    pub fn save(self, path: &str) -> Result<()> {
        let yaml = Self::json_to_yaml(self.json)?;

        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        emitter.dump(&yaml)?;
        fs::write(path, yaml_string.as_bytes())?;
        Ok(())
    }

    pub fn calc_digest(&self) -> String {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        self.update_digest(&self.json, &mut hasher);
        hex::encode(hasher.finish())
    }

    pub fn merge(self, _from: &Config) -> Self {
        // TODO: Implement
        self
    }
}

// Private methods
impl Config {
    fn json_to_yaml(json: Value) -> Result<Yaml> {
        Ok(match json {
            Value::Null => Yaml::Null,
            Value::Bool(b) => Yaml::Boolean(b),
            Value::Number(n) => Yaml::Real(n.to_string()),
            Value::String(s) => Yaml::String(s),
            Value::Array(a) => {
                let mut arr = Vec::new();
                for v in a {
                    arr.push(Self::json_to_yaml(v)?);
                }
                Yaml::Array(arr)
            }
            Value::Object(o) => {
                let mut hash = yaml::Hash::new();
                for (k, v) in o {
                    hash.insert(Yaml::from_str(&k), Self::json_to_yaml(v)?);
                }
                Yaml::Hash(hash)
            }
        })
    }

    fn yaml_to_json(yaml: Yaml) -> Result<Value> {
        Ok(match yaml {
            Yaml::Real(s) => Value::Number(Number::from_f64(s.parse().unwrap()).unwrap()),
            Yaml::Integer(i) => Value::Number(Number::from_f64(i as f64).unwrap()),
            Yaml::String(s) => Value::String(s),
            Yaml::Boolean(b) => Value::Bool(b),
            Yaml::Array(a) => {
                let mut arr = Vec::new();
                for v in a {
                    arr.push(Self::yaml_to_json(v)?);
                }
                Value::Array(arr)
            }
            Yaml::Hash(h) => {
                let mut map = Map::new();
                for (k, v) in h {
                    ensure!(k.as_str().is_some(), QedaError::InvalidConfig);
                    map.insert(k.into_string().unwrap(), Self::yaml_to_json(v)?);
                }
                Value::Object(map)
            }
            Yaml::Null => Value::Null,
            _ => Value::default(),
        })
    }

    fn update_digest(&self, element: &Value, hasher: &mut Hasher) {
        match element {
            Value::String(s) => hasher.write_all(s.as_bytes()).unwrap(),
            Value::Number(n) => hasher
                .write_all(&n.as_f64().unwrap().to_le_bytes())
                .unwrap(),
            Value::Bool(b) => hasher.write_all(&(*b as u8).to_le_bytes()).unwrap(),
            Value::Array(a) => {
                for e in a {
                    self.update_digest(e, hasher);
                }
            }
            Value::Object(o) => {
                let keys = o.keys();
                for key in keys {
                    hasher.write_all(key.as_str().as_bytes()).unwrap();
                    self.update_digest(o.get(key).unwrap(), hasher);
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element() {
        let config = Config::from_yaml(
            r"
            A:
              B:
                C: 1
            ",
        )
        .unwrap();

        assert!(config.get_element("A.B.C").is_ok());
        assert!(config.get_element("A.B.D").is_err());
        assert!(config.get_element("B").is_err());
    }

    #[test]
    fn range() -> Result<()> {
        let config = Config::from_yaml(
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

    #[test]
    fn json() -> Result<()> {
        let config = Config::from_json(
            r#"{
        "A": 1,
        "B": 1.3,
        "C": "str",
        "D": [5, 6],
        "E": { "E1": 1, "E2": false },
        "F": [{ "F1": null }, { "F2": "null" } ],
        "G": true
            }"#,
        )?;

        assert_eq!(config.get_i64("A")?, 1);
        assert_eq!(config.get_f64("B")?, 1.3);
        assert_eq!(config.get_str("C")?, "str");
        assert_eq!(config.get_bool("G")?, true);

        Ok(())
    }
}
