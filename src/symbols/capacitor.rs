use yaml_rust::Yaml;

use crate::errors::*;
use crate::schematic::Symbol;
use crate::drawing::Drawing;

pub struct CapacitorSymbol {}

impl Symbol for CapacitorSymbol {
    fn draw(&self, _config: &Yaml)-> Result<Drawing> {
        debug!("draw capacitor symbol");
        let mut drawing = Drawing::new();
        drawing.add_line(0, 0, 0, 0);
        Ok(drawing)
    } 
}