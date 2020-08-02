use std::collections::HashMap;

use yaml_rust::Yaml;

use crate::config::Config;
use crate::drawing::Element;

pub struct Pinout {
    pins: HashMap<String, Vec<String>>,
}

impl Pinout {
    pub fn from_config(config: &Config) -> Self {
        let mut pinout = Pinout {
            pins: HashMap::new(),
        };
        if let Ok(config_pinout) = config.get_hash("pinout") {
            for (pin, numbers) in config_pinout {
                if let Yaml::String(pin) = pin {
                    let numbers = match numbers {
                        Yaml::Integer(number) => vec!(number.to_string()),
                        Yaml::Array(numbers) => numbers.iter().filter_map(
                            |number| match number {
                                Yaml::Integer(number) => Some(number.to_string()),
                                _ => None,
                            }
                        ).collect(),
                        Yaml::String(numbers) => numbers.split(',').map(|number| number.to_string()).collect(),
                        _ => vec!(),
                    };
                    pinout.pins.insert(pin.to_string(), numbers);
                }
            }
        }
        pinout
    }

    pub fn add_default(&mut self, pin: &str, number: &str) {
        if self.pins.contains_key(pin) {
            return
        }
        self.pins.insert(pin.to_string(), vec!(number.to_string()));
    }

    pub fn apply_to(&self, elements: &mut Vec<Element>) {
        let mut pin_counter = HashMap::new();
        for element in elements {
            if let Element::Pin(pin) = element {
                if let Some(numbers) = self.pins.get(&pin.net) {
                    let index = *pin_counter.get(&pin.net).unwrap_or(&(0 as usize));
                    pin.number = numbers.get(index).unwrap().to_string();
                    pin_counter.insert(&pin.net, index + 1);
                }
            }
        }
    }
}
