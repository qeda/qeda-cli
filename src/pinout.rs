#![allow(dead_code)] // TODO: Remove

use linked_hash_map::LinkedHashMap;
use regex::Regex;
use yaml_rust::Yaml;

use crate::config::Config;
use crate::error::*;

bitflags! {
    pub struct PinKind: u16 {
        const UNSPECIFIED    = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const TRISTATE       = 0x0004;
        const PASSIVE        = 0x0008;
        const POWER          = 0x0010;
        const OPEN_COLLECTOR = 0x0020;
        const OPEN_EMITTER   = 0x0040;
        const NOT_CONNECTED  = 0x0080;
    }
}
bitflags! {
    pub struct PinShape: u16 {
        const LINE           = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const INVERTED       = 0x0004;
        const CLOCK          = 0x0008;
        const LOW            = 0x0010;
        const FALLING_EDGE   = 0x0020;
        const NON_LOGIC      = 0x0040;
    }
}

#[derive(Debug)]
pub struct Pin {
    pub name: String,
    pub number: String,
    pub kind: PinKind,
    pub shape: PinShape,
}

impl Pin {
    pub fn new(name: &str, number: &str) -> Self {
        Pin {
            name: name.to_string(),
            number: number.to_string(),
            kind: PinKind::UNSPECIFIED,
            shape: PinShape::LINE,
        }
    }
}

#[derive(Debug)]
struct Pinout {
    pins: Vec<Pin>,
    groups: LinkedHashMap<String, Vec<usize>>,
    letters: Vec<String>,
}

impl Pinout {
    pub fn new() -> Self {
        let mut letters: Vec<String> = vec![
            "", "A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "L", "M", "N", "P", "R", "T",
            "U", "V", "W", "Y", "Z",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let len = letters.len();
        for i in 1..len {
            for j in i..len {
                letters.push(format!("{}{}", letters[i], letters[j]));
            }
        }

        Pinout {
            pins: Vec::new(),
            groups: LinkedHashMap::new(),
            letters: letters,
        }
    }

    pub fn add_pin(&mut self, pin: Pin) -> Vec<usize> {
        let mut result = Vec::new();
        let index = self.pins.len();
        if !self.groups.contains_key(&pin.name) {
            self.groups.insert(pin.name.clone(), Vec::new());
        }
        let group = self.groups.get_mut(&pin.name).unwrap();
        group.push(index);
        result.push(index);
        self.pins.push(pin);
        result
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        let mut result = Self::new();
        if let Ok(pinout_yaml) = config.get_element("pinout") {
            match pinout_yaml {
                Yaml::Hash(h) => {
                    for (name, value) in h {
                        result.add_pins(name, value)?;
                    }
                }
                _ => (), // TODO: Return the error about unexpected type
            }
        }
        Ok(result)
    }
}

impl Pinout {
    fn add_pins(&mut self, name: &Yaml, value: &Yaml) -> Result<Vec<usize>> {
        let mut result = Vec::new();
        match value {
            Yaml::Hash(h) => {
                for (key, value) in h {
                    result.append(&mut self.add_pins(key, value)?);
                }
                if let Yaml::String(ref s) = name {
                    if !self.groups.contains_key(s) {
                        self.groups.insert(s.clone(), Vec::new());
                    }
                    let group = self.groups.get_mut(s).unwrap();
                    group.append(&mut result); // TODO: Consider `result.clone()` if the result will be used somewhere
                }
            }
            Yaml::Integer(_) | Yaml::String(_) | Yaml::Array(_) => {
                let names = self.parse_name(name)?;
                let numbers = self.parse_number(value)?;
                if names.len() > 1 {
                    // Multiple names to multiple numbers
                    ensure!(
                        names.len() == numbers.len(),
                        QedaError::InvalidPinCount(names.join(", "), numbers.join(", "))
                    );
                    for i in 0..names.len() {
                        let pin = Pin::new(&names[i], &numbers[i]);
                        result.append(&mut self.add_pin(pin));
                    }
                } else if names.len() == 1 {
                    // Single name to multiple numbers
                    for i in 0..numbers.len() {
                        let pin = Pin::new(&names[0], &numbers[i]);
                        result.append(&mut self.add_pin(pin));
                    }
                }
            }
            _ => (), // TODO: Return the error about unexpected type
        };
        Ok(result)
    }

    fn parse_name(&self, name: &Yaml) -> Result<Vec<String>> {
        let mut result = Vec::new();
        match name {
            Yaml::String(s) => {
                let re = Regex::new(r"(\D*)(\d+)\s*\.\.\s*(\D*)(\d+)").unwrap();
                if re.is_match(&s) {
                    let caps = re
                        .captures(s)
                        .ok_or(QedaError::InvalidPinName(s.to_string()))?;
                    ensure!(
                        caps[1].eq(&caps[3]),
                        QedaError::InvalidPinRangeNameBase(
                            s.to_string(),
                            caps[1].to_string(),
                            caps[3].to_string()
                        )
                    );
                    let begin = caps[2].parse::<usize>()?;
                    let end = caps[4].parse::<usize>()?;
                    ensure!(begin < end, QedaError::InvalidPinName(s.to_string()));
                    for i in begin..=end {
                        ensure!(begin < end, QedaError::InvalidPinName(s.to_string()));
                        result.push(format!("{}{}", &caps[1], i));
                    }
                } else {
                    result.push(s.to_string());
                }
            }
            Yaml::Array(a) => {
                for name in a {
                    let mut sub_names = self.parse_name(name)?;
                    result.append(&mut sub_names);
                }
            }
            _ => (), // TODO: Return the error about unexpected type
        }
        Ok(result)
    }

    fn parse_number(&self, number: &Yaml) -> Result<Vec<String>> {
        let mut result = Vec::new();
        match number {
            Yaml::Integer(i) => {
                result.push(i.to_string());
            }
            Yaml::String(s) => {
                let s = s.to_uppercase();
                let re = Regex::new(r"([A-Z]{0,2})(\d+)\s*\.\.\s*([A-Z]{0,2})(\d+)").unwrap();
                if re.is_match(&s) {
                    let caps = re
                        .captures(&s)
                        .ok_or(QedaError::InvalidPinNumber(s.to_string()))?;
                    let row_begin = self
                        .letters
                        .iter()
                        .position(|s| s.eq(&caps[1]))
                        .ok_or(QedaError::InvalidPinNumber(s.to_string()))?;
                    let mut row_end = self
                        .letters
                        .iter()
                        .position(|s| s.eq(&caps[3]))
                        .ok_or(QedaError::InvalidPinNumber(s.to_string()))?;
                    if row_end < row_begin {
                        row_end = row_begin;
                    }
                    let col_begin = caps[2].parse::<usize>()?;
                    let col_end = caps[4].parse::<usize>()?;
                    ensure!(
                        col_begin < col_end,
                        QedaError::InvalidPinNumber(s.to_string())
                    );
                    for row in row_begin..=row_end {
                        for col in col_begin..=col_end {
                            result.push(format!("{}{}", self.letters[row], col));
                        }
                    }
                } else {
                    result.push(s.to_string());
                }
            }
            Yaml::Array(a) => {
                for number in a {
                    let mut sub_numbers = self.parse_number(number)?;
                    result.append(&mut sub_numbers);
                }
            }
            _ => (), // TODO: Return the error about unexpected type
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinout() -> Result<()> {
        let pinout_yaml = r"
        pinout:
          A: 1
          B: 2..3
          C: [4, 5, 6, 11 .. 13]
          D0 .. D1: [A7..A8]
          [E, F, G0..G2]: [9, 10, 20..22]
        ";
        let pinout = Pinout::from_config(&Config::from_str(pinout_yaml)?)?;
        assert!(pinout.groups.contains_key("A"));
        assert!(pinout.groups.contains_key("B"));
        assert!(pinout.groups.contains_key("C"));
        assert!(pinout.groups.contains_key("D0"));
        assert!(pinout.groups.contains_key("D1"));
        assert!(pinout.groups.contains_key("E"));
        assert!(pinout.groups.contains_key("F"));
        assert!(pinout.groups.contains_key("G0"));
        assert!(pinout.groups.contains_key("G1"));
        assert!(pinout.groups.contains_key("G2"));

        assert_eq!(*pinout.groups.get("A").unwrap(), vec!(0));
        assert_eq!(*pinout.groups.get("B").unwrap(), vec!(1, 2));
        assert_eq!(*pinout.groups.get("C").unwrap(), vec!(3, 4, 5, 6, 7, 8));
        assert_eq!(*pinout.groups.get("D0").unwrap(), vec!(9));
        assert_eq!(*pinout.groups.get("D1").unwrap(), vec!(10));
        assert_eq!(*pinout.groups.get("E").unwrap(), vec!(11));
        assert_eq!(*pinout.groups.get("F").unwrap(), vec!(12));
        assert_eq!(*pinout.groups.get("G0").unwrap(), vec!(13));
        assert_eq!(*pinout.groups.get("G1").unwrap(), vec!(14));
        assert_eq!(*pinout.groups.get("G2").unwrap(), vec!(15));

        Ok(())
    }

    #[test]
    fn test_pinout_group() -> Result<()> {
        let pinout_yaml = r"
        pinout:
          GROUP:
            A: 1
            B: 2..3
        ";
        let pinout = Pinout::from_config(&Config::from_str(pinout_yaml)?)?;
        assert!(pinout.groups.contains_key("GROUP"));
        assert!(pinout.groups.contains_key("A"));
        assert!(pinout.groups.contains_key("B"));

        assert_eq!(*pinout.groups.get("GROUP").unwrap(), vec!(0, 1, 2));
        assert_eq!(*pinout.groups.get("A").unwrap(), vec!(0));
        assert_eq!(*pinout.groups.get("B").unwrap(), vec!(1, 2));

        Ok(())
    }
}
