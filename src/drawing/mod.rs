pub mod prelude;

mod attribute;
mod svg;
mod symbol_pin;

use regex::Regex;
use std::collections::HashMap;

use crate::error::*;
use crate::geometry::*;
use crate::pinout::Pinout;

pub use attribute::Attribute;
pub use prelude::*;
pub use symbol_pin::SymbolPin;

use svg::*;

#[derive(Debug)]
pub enum Element {
    Attribute(Attribute),
    Line(Line),
    SymbolPin(SymbolPin),
}

#[derive(Debug)]
pub struct Drawing {
    pub elements: Vec<Element>,

    canvas_transform: Transformation,
    attrs: HashMap<String, String>,
}

impl Drawing {
    /// Creates a new drawing.
    pub fn new() -> Drawing {
        Drawing {
            canvas_transform: Transformation::new(),
            elements: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    /// Creates a drawing from the SVG string.
    pub fn from_svg(svg: &str, pinout: Pinout) -> Result<Drawing> {
        let mut drawing = Drawing::new();
        drawing.add_svg(svg, pinout)?;
        Ok(drawing)
    }

    /// Adds drawing elements from the SVG string.
    pub fn add_svg(&mut self, svg: &str, pinout: Pinout) -> Result<()> {
        let mut elements = svg::to_elements(svg)?;
        let mut sx = 1.0;
        let mut sy = 1.0;
        let mut dx = 0.0;
        let mut dy = 0.0;
        if let Some(SvgElement::HLine(ch)) = elements.remove("ch") {
            sx = 1.0 / ch.len();
            dx = -ch.cx();
        }
        if let Some(SvgElement::VLine(cv)) = elements.remove("cv") {
            sy = 1.0 / cv.len();
            dy = -cv.cy();
        }
        self.canvas_transform.translate(dx, dy);
        // SVG has y axis directed downwards. We need to turn it upwards
        self.canvas_transform.scale(sx, -sy);

        debug!("SVG elements: {:?}", &elements);
        for (id, element) in elements {
            match element {
                SvgElement::HLine(hline) => {
                    let line = Line::new(hline.x0, hline.y, hline.x1, hline.y);
                    if id.starts_with("pin") {
                        self.add_symbol_pin(&id, &pinout, line)?;
                    } else {
                        self.add_line(line.width(hline.width));
                    }
                }
                SvgElement::VLine(vline) => {
                    let line = Line::new(vline.x, vline.y0, vline.x, vline.y1);
                    if id.starts_with("pin") {
                        self.add_symbol_pin(&id, &pinout, line)?;
                    } else {
                        self.add_line(line.width(vline.width));
                    }
                }
                SvgElement::Text(text) => self.add_attribute(&id, text),
                _ => (),
            }
        }
        debug!("Elements: {:?}", &self.elements);
        Ok(())
    }

    /// Adds a line object to the drawing.
    pub fn add_line(&mut self, mut line: Line) {
        line.transform(&self.canvas_transform);
        self.elements.push(Element::Line(line));
    }

    /// Adds an attribute object to the drawing.
    pub fn add_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_string(), value.to_string());
    }

    /// Returns the attribute value.
    pub fn attr(&self, key: &str, def: &str) -> String {
        self.attrs.get(key).unwrap_or(&def.to_string()).clone()
    }
}

// Private methods
impl Drawing {
    fn add_attribute(&mut self, id: &str, text: SvgText) {
        let mut attr = Attribute::new(id)
            .origin(text.x, text.y)
            .font_size(text.height)
            .align(text.halign, text.valign);
        attr.transform(&self.canvas_transform);
        self.elements.push(Element::Attribute(attr));
    }

    fn add_symbol_pin(&mut self, id: &str, pinout: &Pinout, mut line: Line) -> Result<()> {
        let id_elems: Vec<&str> = id.split(':').collect();
        ensure!(
            id_elems.len() == 3,
            QedaError::InvalidSvgPinId(id.to_string())
        );

        let name = id_elems[0];
        let halign = id_elems[1];
        let valign = id_elems[2];

        let re = Regex::new(r"^pin-(.*)$").unwrap();
        let caps = re
            .captures(name)
            .ok_or(QedaError::InvalidSvgPinName(name.to_string()))?;
        ensure!(caps.len() > 1, QedaError::InvalidSvgPinId(name.to_string()));

        let name = &caps[1];
        let halign = HAlign::from_str(halign);
        let valign = VAlign::from_str(valign);

        line.transform(&self.canvas_transform);

        let pin = pinout
            .get_first(name)
            .ok_or(QedaError::InvalidSvgPinName(name.to_string()))?;
        let sym_pin = SymbolPin::new(pin.clone(), halign, valign, &line);
        self.elements.push(Element::SymbolPin(sym_pin));

        Ok(())
    }
}
