use std::fs;
use std::io::Write;
use std::path::Path;

use crypto_hash::{Algorithm, Hasher};
use regex::Regex;
use serde_json::{Map, Number, Value};
use yaml_rust::{yaml, Yaml, YamlEmitter, YamlLoader};

use crate::error::*;

#[derive(Debug, Default)]
pub struct Range(pub f64, pub f64);

impl Range {
    #[inline]
    pub fn max(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn min(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn nom(&self) -> f64 {
        (self.0 + self.1) / 2.0
    }

    #[inline]
    pub fn tol(&self) -> f64 {
        self.1 - self.0
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}

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

    /// Calculates the `Config` digest (a.k.a. fingerprint).
    pub fn calc_digest(&self) -> String {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        self.update_digest(&self.json, &mut hasher);
        hex::encode(hasher.finish())
    }

    /// Creates a new file if it doesn't exist.
    pub fn create_if_missing(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::write(path, b"---")?;
        }
        Ok(())
    }

    /// Creates a new `Config` from JSON string.
    pub fn from_json(json: &str) -> Result<Config> {
        let value: Value = serde_json::from_str(json)?;
        Ok(Config { json: value })
    }

    /// Creates a new `Config` from YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Config> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(!docs.is_empty(), QedaError::InvalidConfig);
        let mut json = Self::yaml_to_json(docs.pop().unwrap())?;
        if json == Value::Null {
            json = Value::Object(Map::new());
        }
        Ok(Config { json })
    }

    /// Creates a new `Config` from YAML file.
    pub fn from_yaml_file(path: &str) -> Result<Config> {
        Self::from_yaml(&fs::read_to_string(path)?)
    }

    /// Returns a `bool` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        Ok(self
            .get_element(key)?
            .as_bool()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "bool"))?)
    }

    /// Returns a config element.
    pub fn get_element(&self, key: &str) -> Result<&Value> {
        let keys: Vec<&str> = key.split('.').collect();
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

    /// Returns a `f64` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_f64(&self, key: &str) -> Result<f64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "number"))?)
    }

    /// Returns a `i64` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_i64(&self, key: &str) -> Result<i64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "number"))?
            .round() as i64)
    }

    /// Returns an `Object` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_object(&self, key: &str) -> Result<&Map<String, Value>> {
        Ok(self
            .get_element(key)?
            .as_object()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "object"))?)
    }

    /// Returns a pair config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_pair(&self, key: &str) -> Result<(f64, f64)> {
        let elem = self.get_element(key)?;
        match elem {
            Value::Number(n) => {
                let f = n.as_f64().unwrap();
                Ok((f, f))
            }
            Value::String(s) => {
                let re = Regex::new(r"(\d+\.*\d*)\s*;\s*(\d+\.*\d*)").unwrap();
                let caps = re.captures(s).ok_or_else(|| {
                    QedaError::InvalidElementType(key.to_string(), "pair: f64;f64")
                })?;
                let f1 = caps[1].parse::<f64>()?;
                let f2 = caps[2].parse::<f64>()?;
                Ok((f1, f2))
            }
            Value::Array(a) if a.len() == 2 => {
                let f1 = a[0]
                    .as_f64()
                    .or_else(|| a[0].as_i64().map(|v| v as f64))
                    .ok_or_else(|| {
                        QedaError::InvalidElementType(key.to_string(), "pair: [f64, f64]")
                    })?;
                let f2 = a[1]
                    .as_f64()
                    .or_else(|| a[1].as_i64().map(|v| v as f64))
                    .ok_or_else(|| {
                        QedaError::InvalidElementType(key.to_string(), "pair: [f64, f64]")
                    })?;
                Ok((f1, f2))
            }
            _ => Err(QedaError::InvalidElementType(key.to_string(), "pair").into()),
        }
    }

    /// Returns a range config value.
    ///
    /// Returns an error if there is no parameter with the specified key or
    /// a range has invalid format.
    pub fn get_range(&self, key: &str) -> Result<Range> {
        let elem = self.get_element(key)?;
        match elem {
            Value::Number(n) => {
                let f = n.as_f64().unwrap();
                Ok(Range(f, f))
            }
            Value::String(s) => {
                let re = Regex::new(r"(\d+\.*\d*)\s*(\.\.|\+/-)\s*(\d+\.*\d*)").unwrap();
                let caps = re.captures(s).ok_or_else(|| {
                    QedaError::InvalidElementType(key.to_string(), "range: f64..f64 or f64 +/- f64")
                })?;
                let f1 = caps[1].parse::<f64>()?;
                let f2 = caps[3].parse::<f64>()?;
                match &caps[2] {
                    ".." => Ok(Range(f1, f2)),
                    "+/-" => Ok(Range(f1 - f2, f1 + f2)),
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
                    .or_else(|| a[1].as_i64().map(|v| v as f64))
                    .ok_or_else(|| {
                        QedaError::InvalidElementType(key.to_string(), "range: [f64, f64]")
                    })?;
                let f2 = a[1]
                    .as_f64()
                    .or_else(|| a[1].as_i64().map(|v| v as f64))
                    .ok_or_else(|| {
                        QedaError::InvalidElementType(key.to_string(), "range: [f64, f64]")
                    })?;
                Ok(Range(f1, f2))
            }
            _ => Err(QedaError::InvalidElementType(key.to_string(), "range").into()),
        }
    }

    /// Returns a `str` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_str(&self, key: &str) -> Result<&str> {
        Ok(self
            .get_element(key)?
            .as_str()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "string"))?)
    }

    /// Returns a `String` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_string(&self, key: &str) -> Result<String> {
        Ok(self
            .get_element(key)?
            .as_str()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "string"))?
            .to_string())
    }

    /// Returns a `u64` config value.
    ///
    /// Returns an error if there is no parameter with the specified key.
    pub fn get_u64(&self, key: &str) -> Result<u64> {
        Ok(self
            .get_element(key)?
            .as_f64()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "number"))?
            .round() as u64)
    }

    /// Inserts (or replaces) a specified value.
    pub fn insert(&mut self, key: &str, value: Value) {
        let keys = key.split('.');
        let mut element = &mut self.json;
        for key in keys {
            if !element[key].is_null() {
                element[key] = Value::Object(Map::new());
            }
            element = &mut element[key];
        }
        *element = value;
    }

    /// Inserts an object to the `Config`.
    pub fn insert_object(&mut self, key: &str, name: &str) -> Result<()> {
        let map = self.json.as_object_mut().unwrap();
        if !map.contains_key(key) {
            // Insert child if doesn't exist
            map.insert(key.to_string(), Value::Object(Map::new()));
        }
        let child = map[key]
            .as_object_mut()
            .ok_or_else(|| QedaError::InvalidElementType(key.to_string(), "object"))?;
        child.insert(name.to_string(), Value::Object(Map::new()));
        Ok(())
    }

    /// Merges the `Config`with another.
    pub fn merge(mut self, with: &Config) -> Self {
        Self::merge_objects(&mut self.json, &with.json);
        self
    }

    /// Saves the `Config` to YAML file.
    pub fn save(self, path: &str) -> Result<()> {
        let yaml = Self::json_to_yaml(self.json)?;

        let mut yaml_string = String::new();
        let mut emitter = YamlEmitter::new(&mut yaml_string);
        emitter.dump(&yaml)?;
        fs::write(path, yaml_string.as_bytes())?;
        Ok(())
    }

    // Convert JSON to YAML
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

    // Merge two objects.
    fn merge_objects(to: &mut Value, from: &Value) {
        match from {
            Value::Array(from) => {
                if let Value::Array(to) = to {
                    to.append(&mut from.clone());
                }
            }
            Value::Bool(n) => *to = Value::Bool(*n),
            Value::Number(n) => *to = Value::Number(n.clone()),
            Value::Object(from) => {
                for key in from.keys() {
                    if to[key].is_null() {
                        to[key] = from[key].clone();
                    } else {
                        Self::merge_objects(&mut to[key], &from[key]);
                    }
                }
            }
            Value::String(s) => *to = Value::String(s.clone()),
            _ => (),
        }
    }

    // Convert YAML to JSON
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

impl Default for Config {
    /// Creates an empty `Config`.
    #[inline]
    fn default() -> Self {
        Self::new()
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
        assert_eq!(config.get_i64("A.B.C").unwrap(), 1);
        assert_eq!(config.get_u64("A.B.C").unwrap(), 1);
        assert_eq!(config.get_f64("A.B.C").unwrap(), 1.0);
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

        assert_eq!(config.get_range("A")?, Range(1.0, 1.0));
        assert_eq!(config.get_range("B")?, Range(1.3, 1.3));
        assert_eq!(config.get_range("C")?, Range(1.0, 2.0));
        assert_eq!(config.get_range("D")?, Range(5.0, 6.0));
        assert_eq!(config.get_range("E")?, Range(3.9, 6.1));
        assert_eq!(config.get_range("F")?, Range(1.0, 2.0));
        assert_eq!(config.get_range("G")?, Range(2.25, 6.5));
        Ok(())
    }

    #[test]
    fn pair() -> Result<()> {
        let config = Config::from_yaml(
            r"
        A: 1
        B: 1.3
        C: 1.0;2.0
        D: 5 ; 6
        E: [1, 2]
        F: [2.25, 6.5]
        ",
        )?;

        assert_eq!(config.get_pair("A")?, (1.0, 1.0));
        assert_eq!(config.get_pair("B")?, (1.3, 1.3));
        assert_eq!(config.get_pair("C")?, (1.0, 2.0));
        assert_eq!(config.get_pair("D")?, (5.0, 6.0));
        assert_eq!(config.get_pair("E")?, (1.0, 2.0));
        assert_eq!(config.get_pair("F")?, (2.25, 6.5));
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
