use crate::drawing::{Drawing, Transform, Transformation};

#[derive(Debug, Default)]
pub struct Symbol {
    pub ref_des: String,
    pub parts: Vec<Drawing>,
    pub show_pin_numbers: bool,
    pub show_pin_names: bool,
    pub power: bool,
}

impl Symbol {
    /// Creates a new `Symbol`.
    pub fn new() -> Self {
        Symbol {
            ref_des: "U".to_string(),
            parts: Vec::new(),
            show_pin_numbers: false,
            show_pin_names: false,
            power: false,
        }
    }

    /// Adds part to the `Symbol`.
    pub fn add_part(&mut self, part: Drawing) {
        if let Some(ref_des) = part.find_attribute("ref-des") {
            self.ref_des = ref_des.value.clone();
        }
        self.parts.push(part);
    }
}

impl Transform for Symbol {
    fn transform(mut self, t: &Transformation) -> Self {
        self.parts = self.parts.into_iter().map(|p| p.transform(t)).collect();
        self
    }
}
