use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use crate::component::Component;
use crate::config::Config;
use crate::drawing::*;
use crate::error::Result;

#[derive(Default)]
pub struct KicadFootprints {}

impl fmt::Display for PadShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PadShape::Circle => write!(f, "circle"),
            PadShape::Rect => write!(f, "rect"),
            PadShape::RoundRect => write!(f, "ruondrect"),
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut layers: Vec<&str> = Vec::new();
        if self.contains(Layer::COPPER_TOP | Layer::COPPER_BOTTOM) {
            layers.push("*.Cu");
        } else if self.contains(Layer::COPPER_TOP) {
            layers.push("F.Cu");
        } else if self.contains(Layer::COPPER_BOTTOM) {
            layers.push("B.Cu");
        }

        if self.contains(Layer::SILK_TOP | Layer::SILK_BOTTOM) {
            layers.push("*.SilkS");
        } else if self.contains(Layer::SILK_TOP) {
            layers.push("F.SilkS");
        } else if self.contains(Layer::SILK_BOTTOM) {
            layers.push("B.SilkS");
        }

        if self.contains(Layer::MASK_TOP | Layer::MASK_BOTTOM) {
            layers.push("*.Mask");
        } else if self.contains(Layer::MASK_TOP) {
            layers.push("F.Mask");
        } else if self.contains(Layer::MASK_BOTTOM) {
            layers.push("B.Mask");
        }

        if self.contains(Layer::PASTE_TOP | Layer::PASTE_BOTTOM) {
            layers.push("*.Paste");
        } else if self.contains(Layer::PASTE_TOP) {
            layers.push("F.Paste");
        } else if self.contains(Layer::PASTE_BOTTOM) {
            layers.push("B.Paste");
        }

        write!(f, "{}", layers.join(" "))
    }
}

impl KicadFootprints {
    pub fn render(&mut self, components: &Vec<Component>, _config: &Config) -> Result<()> {
        for component in components {
            let name = &component.name;
            let pattern = &component.pattern;
            info!("  • {}", name);
            let mut f = File::create(format!("{}.kicad_mod", name))?;
            writeln!(f, "(module {name} (layer F.Cu)", name = name)?;
            for element in &pattern.elements {
                match element {
                    Element::Pad(p) => {
                        writeln!(
                            f,
                            "  (pad  {name} {kind} {shape} (at {x} {y}) (size {sx} {sy}) (layers {layers}))",
                            name = p.name,
                            kind = if p.is_smd() { "smd" } else { "thru_hole" },
                            shape = p.shape,
                            x = p.origin.x,
                            y = p.origin.y,
                            sx = p.size.x,
                            sy = p.size.y,
                            layers = p.layers,
                        )?;
                    }
                    _ => (),
                }
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}
