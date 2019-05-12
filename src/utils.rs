use std::io::Write;

use crypto_hash::{Algorithm, Hasher};
use hex;
use yaml_rust::Yaml;

use crate::errors::*;

fn update_digest(element: &Yaml, hasher: &mut Hasher) {
    match element {
        Yaml::Real(s) | Yaml::String(s) => hasher.write_all(s.as_bytes()).unwrap(),
        Yaml::Integer(i) => hasher.write_all(&i.to_le_bytes()).unwrap(),
        Yaml::Boolean(b) => {
            let b = *b as u8;
            hasher.write_all(&b.to_le_bytes()).unwrap()
        },
        Yaml::Array(a) => {
            for e in a {
                update_digest(e, hasher);
            }
        },
        Yaml::Hash(h)=> {
            let keys = h.keys();
            for key in keys {
                hasher.write_all(key.as_str().unwrap().as_bytes()).unwrap();
                update_digest(h.get(key).unwrap(), hasher);
            }
        },
        Yaml::Alias(u) => hasher.write_all(&u.to_le_bytes()).unwrap(),
        _ => {},
    }
}

pub fn calc_digest(config: &Yaml) -> String {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    update_digest(config, &mut hasher);
    hex::encode(hasher.finish())
}

pub fn get_yaml_element<'a, 'b>(key: &'a str, config: &'b Yaml)-> Result<&'b Yaml> {
    let keys: Vec<&str> = key.split(".").collect();
    let mut element = &config[keys[0]];
    for key in &keys[1..] {
        element = &element[*key];
    }
    if element.is_badvalue() {
        Err(ErrorKind::MissingElement(key.to_string()).into())
    } else {
        Ok(element)
    }
}

pub fn get_yaml_string(key: &str, config: &Yaml)-> Result<String> {
    Ok(get_yaml_element(key, config)?
        .as_str()
        .ok_or(ErrorKind::InvalidElementType(key.to_string(), "string".to_string()))?
        .to_string()
    )
}