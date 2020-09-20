use linked_hash_map::LinkedHashMap;
use regex::Regex;
use serde_json::Value;

use crate::config::Config;
use crate::error::*;

// Pin properties: electrical type
bitflags! {
    pub struct PinKind: u16 {
        const UNSPECIFIED    = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const HI_Z           = 0x0004; // Also called `tristate`
        const PASSIVE        = 0x0008;
        const POWER          = 0x0010;
        const OPEN_COLLECTOR = 0x0020;
        const OPEN_DRAIN     = 0x0020;
        const OPEN_EMITTER   = 0x0040;
        const OPEN_SOURCE    = 0x0040;
        const NOT_CONNECTED  = 0x0080;
    }
}

// Pin properties: decoration
bitflags! {
    pub struct PinShape: u16 {
        const LINE           = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const INVERTED       = 0x0004;
        const CLOCK          = 0x0008;
        const ACTIVE_LOW     = 0x0010;
        const NON_LOGIC      = 0x0020;
        const ANALOG         = 0x0040;
        const PULL_UP        = 0x0080;
        const PULL_DOWN      = 0x0100;
        const POSTPONED      = 0x0200;
        const SHIFT          = 0x0400;

    }
}

#[derive(Clone, Debug)]
pub struct Pin {
    pub name: String,
    pub number: String,
    pub kind: PinKind,
    pub shape: PinShape,
}

impl Pin {
    /// Creates a new empty `Pin`.
    pub fn new(name: &str, number: &str) -> Self {
        Pin {
            name: name.to_string(),
            number: number.to_string(),
            kind: PinKind::UNSPECIFIED,
            shape: PinShape::LINE,
        }
    }

    /// Sets `Pin`'s electrical type (`kind`).
    pub fn kind(mut self, kind: PinKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets `Pin`'s decoration style (`shape`).
    pub fn shape(mut self, shape: PinShape) -> Self {
        self.shape = shape;
        self
    }
}

#[derive(Debug)]
pub struct Pinout {
    pub pins: Vec<Pin>,
    pub groups: LinkedHashMap<String, Vec<usize>>,
    letters: Vec<String>,
}

impl Pinout {
    /// Creates a new empty `Pinout`.
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
            letters,
        }
    }

    /// Adds `Pin` to the current `Pinout`.
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

    /// Creates a new `Pinout` from the `Config`.
    pub fn from_config(config: &Config) -> Result<Self> {
        let mut result = Self::new();
        if let Ok(pinout_value) = config.get_element("pinout") {
            match pinout_value {
                Value::Object(o) => {
                    for (k, v) in o {
                        result.add_pins(k, v)?;
                    }
                }
                Value::String(_) => (), // TODO: Parse range `1..20`
                Value::Array(_) => (),  // TODO: Parse range `[1, 20]`
                _ => (),                // TODO: Return the error about unexpected type
            }
        }
        if let Ok(Value::Object(o)) = config.get_element("pin-properties") {
            for (key, value) in o {
                dbg!(key);
                dbg!(value);
                // TODO: Process pin properties
            }
        }
        Ok(result)
    }

    /// Returns the first `Pin` with the specified `name`.
    pub fn get_first(&self, name: &str) -> Option<&Pin> {
        if self.groups.contains_key(name) {
            let index = *self.groups[name].first().unwrap();
            Some(&self.pins[index])
        } else {
            None
        }
    }

    // Add pins from `Config`'s value
    fn add_pins(&mut self, name: &str, value: &Value) -> Result<Vec<usize>> {
        let mut result = Vec::new();
        match value {
            Value::Object(o) => {
                for (k, v) in o {
                    result.append(&mut self.add_pins(k, v)?);
                }
                if !self.groups.contains_key(name) {
                    self.groups.insert(name.to_string(), Vec::new());
                }
                let group = self.groups.get_mut(name).unwrap();
                group.append(&mut result); // TODO: Consider `result.clone()` if the result will be used somewhere
            }
            Value::Number(_) | Value::String(_) | Value::Array(_) => {
                let names = self.parse_name(name)?;
                let numbers = self.parse_number(value)?;
                if names.len() > 1 && names.len() == numbers.len() {
                    // Multiple names to multiple numbers
                    for i in 0..names.len() {
                        let pin = Pin::new(&names[i], &numbers[i]);
                        result.append(&mut self.add_pin(pin));
                    }
                } else if names.len() == 1 {
                    // Single name to multiple numbers
                    for number in &numbers {
                        let pin = Pin::new(&names[0], number);
                        result.append(&mut self.add_pin(pin));
                    }
                } else {
                    bail!(QedaError::InvalidPinCount(
                        names.join(", "),
                        numbers.join(", ")
                    ));
                }
            }
            _ => (), // TODO: Return the error about unexpected type
        };
        Ok(result)
    }

    // Parse pin name(s) from string
    fn parse_name(&self, name: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();

        let re = Regex::new(r"(\D*)(\d+)\s*\.\.\s*(\D*)(\d+)").unwrap();
        if re.is_match(name) {
            let caps = re
                .captures(name)
                .ok_or_else(|| QedaError::InvalidPinName(name.to_string()))?;
            ensure!(
                caps[1].eq(&caps[3]),
                QedaError::InvalidPinRangeNameBase(
                    name.to_string(),
                    caps[1].to_string(),
                    caps[3].to_string()
                )
            );
            let begin = caps[2].parse::<usize>()?;
            let end = caps[4].parse::<usize>()?;
            ensure!(begin < end, QedaError::InvalidPinName(name.to_string()));
            for i in begin..=end {
                ensure!(begin < end, QedaError::InvalidPinName(name.to_string()));
                result.push(format!("{}{}", &caps[1], i));
            }
        } else {
            result.push(name.to_string());
        }

        Ok(result)
    }

    // Parse pin number(s) from string
    fn parse_number(&self, number: &Value) -> Result<Vec<String>> {
        let mut result = Vec::new();
        match number {
            Value::Number(n) => {
                result.push(n.to_string());
            }
            Value::String(s) => {
                let s = s.to_uppercase();
                let re = Regex::new(r"([A-Z]{0,2})(\d+)\s*\.\.\s*([A-Z]{0,2})(\d+)").unwrap();
                if re.is_match(&s) {
                    let caps = re
                        .captures(&s)
                        .ok_or_else(|| QedaError::InvalidPinNumber(s.to_string()))?;
                    let row_begin = self
                        .letters
                        .iter()
                        .position(|s| s.eq(&caps[1]))
                        .ok_or_else(|| QedaError::InvalidPinNumber(s.to_string()))?;
                    let mut row_end = self
                        .letters
                        .iter()
                        .position(|s| s.eq(&caps[3]))
                        .ok_or_else(|| QedaError::InvalidPinNumber(s.to_string()))?;
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
                    result.push(s);
                }
            }
            Value::Array(a) => {
                for n in a {
                    let mut sub_numbers = self.parse_number(n)?;
                    result.append(&mut sub_numbers);
                }
            }
            _ => (), // TODO: Return the error about unexpected type
        }
        Ok(result)
    }
}

impl Default for Pinout {
    #[inline]
    /// Creates an empty `Pinout`.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinout() -> Result<()> {
        let pinout_yaml = r"
        pinout:
          A: 1
          B: 2..3
          C: [4, 5, 6, 11 .. 13]
          D0 .. D1: [A7..A8]
          E0..E2: 20..22
        ";
        let pinout = Pinout::from_config(&Config::from_yaml(pinout_yaml)?)?;
        assert!(pinout.groups.contains_key("A"));
        assert!(pinout.groups.contains_key("B"));
        assert!(pinout.groups.contains_key("C"));
        assert!(pinout.groups.contains_key("D0"));
        assert!(pinout.groups.contains_key("D1"));
        assert!(pinout.groups.contains_key("E0"));
        assert!(pinout.groups.contains_key("E1"));
        assert!(pinout.groups.contains_key("E2"));

        assert_eq!(*pinout.groups.get("A").unwrap(), vec!(0));
        assert_eq!(*pinout.groups.get("B").unwrap(), vec!(1, 2));
        assert_eq!(*pinout.groups.get("C").unwrap(), vec!(3, 4, 5, 6, 7, 8));
        assert_eq!(*pinout.groups.get("D0").unwrap(), vec!(9));
        assert_eq!(*pinout.groups.get("D1").unwrap(), vec!(10));
        assert_eq!(*pinout.groups.get("E0").unwrap(), vec!(11));
        assert_eq!(*pinout.groups.get("E1").unwrap(), vec!(12));
        assert_eq!(*pinout.groups.get("E2").unwrap(), vec!(13));

        Ok(())
    }

    #[test]
    fn group() -> Result<()> {
        let pinout_yaml = r"
        pinout:
          GROUP:
            A: 1
            B: 2..3
        ";
        let pinout = Pinout::from_config(&Config::from_yaml(pinout_yaml)?)?;
        assert!(pinout.groups.contains_key("GROUP"));
        assert!(pinout.groups.contains_key("A"));
        assert!(pinout.groups.contains_key("B"));

        assert_eq!(*pinout.groups.get("GROUP").unwrap(), vec!(0, 1, 2));
        assert_eq!(*pinout.groups.get("A").unwrap(), vec!(0));
        assert_eq!(*pinout.groups.get("B").unwrap(), vec!(1, 2));

        Ok(())
    }
}
