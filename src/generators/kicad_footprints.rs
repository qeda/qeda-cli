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
            PadShape::RoundRect => write!(f, "roundrect"),
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

        if self.contains(Layer::SILKSCREEN_TOP | Layer::SILKSCREEN_BOTTOM) {
            layers.push("*.SilkS");
        } else if self.contains(Layer::SILKSCREEN_TOP) {
            layers.push("F.SilkS");
        } else if self.contains(Layer::SILKSCREEN_BOTTOM) {
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

        if self.contains(Layer::ASSEMBLY_TOP | Layer::ASSEMBLY_BOTTOM) {
            layers.push("*.Fab");
        } else if self.contains(Layer::ASSEMBLY_TOP) {
            layers.push("F.Fab");
        } else if self.contains(Layer::ASSEMBLY_BOTTOM) {
            layers.push("B.Fab");
        }

        if self.contains(Layer::COURTYARD_TOP | Layer::COURTYARD_BOTTOM) {
            layers.push("*.CrtYd");
        } else if self.contains(Layer::COURTYARD_TOP) {
            layers.push("F.CrtYd");
        } else if self.contains(Layer::COURTYARD_BOTTOM) {
            layers.push("B.CrtYd");
        }

        write!(f, "{}", layers.join(" "))
    }
}

impl KicadFootprints {
    /// Renders footprints.
    pub fn render(self, components: &[Component]) -> Result<()> {
        for component in components {
            let name = &component.name;
            let pattern = &component.pattern;
            info!("  • foorprint: '{}'", name);
            let mut f = File::create(format!("{}.kicad_mod", name))?;
            writeln!(f, "(module {name} (layer F.Cu)", name = name)?;
            for element in &pattern.elements {
                match element {
                    Element::Attribute(a) => {
                        let (kind, value) = match a.id.as_str() {
                            "ref-des" => ("reference", "REF**".to_string()),
                            "value" => ("value", name.clone()),
                            _ => ("user", a.value.clone()),
                        };
                        writeln!(
                            f,
                            "  (fp_text {kind} {value} (at {x:.3} {y:.3}) (layer {layer})",
                            kind = kind,
                            value = value,
                            x = a.origin.x,
                            y = a.origin.y,
                            layer = a.layer,
                        )?;
                        writeln!(f, "    (effects (font (size {font_size:.3} {font_size:.3}) (thickness {line_width:.3})))",
                            font_size = a.font_size,
                            line_width = a.line_width,
                        )?;
                        writeln!(f, "  )")?;
                    }
                    Element::Line(l) => {
                        writeln!(
                            f,
                            "  (fp_line (start {x0:.3} {y0:.3}) (end {x1:.3} {y1:.3}) (layer {layer}) (width {width:.3}))",
                            x0 = l.p.0.x,
                            y0 = l.p.0.y,
                            x1 = l.p.1.x,
                            y1 = l.p.1.y,
                            layer = l.layer,
                            width = l.width,
                        )?;
                    }
                    Element::Pad(p) => {
                        writeln!(
                            f,
                            "  (pad {name} {kind} {shape} (at {x:.3} {y:.3}) (size {sx:.3} {sy:.3}) (layers {layers}) (solder_mask_margin {mask:.3}))",
                            name = p.name,
                            kind = if p.is_smd() { "smd" } else { "thru_hole" },
                            shape = p.shape,
                            x = p.origin.x,
                            y = p.origin.y,
                            sx = p.size.x,
                            sy = p.size.y,
                            layers = p.layers,
                            mask = p.mask,
                        )?;
                    }
                    _ => (),
                }
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }

    /// Builds a `KicadFootprints` with applied settings from `Config`.
    pub fn settings(self, _lib_cfg: &Config) -> Self {
        // TODO: Use settings
        self
    }
}
