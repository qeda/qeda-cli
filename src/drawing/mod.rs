pub mod prelude;
mod symbol_pin;
mod text_box;

use regex::Regex;
use std::collections::HashMap;

use crate::error::*;
use crate::geometry::*;
use crate::pinout::Pinout;
use crate::svg::{self, *};

pub use prelude::*;
pub use symbol_pin::SymbolPin;
pub use text_box::TextBox;

#[derive(Debug)]
pub enum Element {
    Line(Line),
    TextBox(TextBox),
    SymbolPin(SymbolPin),
}

#[derive(Debug)]
pub struct Drawing {
    canvas_transform: Transformation,
    elements: Vec<Element>,
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

    /// Returns all drawing elements.
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    /// Returns all drawing elements as a mutable vector.
    pub fn mut_elements(&mut self) -> &mut Vec<Element> {
        &mut self.elements
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
        for (key, element) in elements {
            match element {
                SvgElement::HLine(hline) => {
                    let line = Line::new(hline.x0, hline.y, hline.x1, hline.y);
                    if key.starts_with("pin") {
                        self.add_symbol_pin(&key, &pinout, line)?;
                    } else {
                        self.add_line(line.width(hline.width));
                    }
                }
                SvgElement::VLine(vline) => {
                    let line = Line::new(vline.x, vline.y0, vline.x, vline.y1);
                    if key.starts_with("pin") {
                        self.add_symbol_pin(&key, &pinout, line)?;
                    } else {
                        self.add_line(line.width(vline.width));
                    }
                }
                SvgElement::Rect(rect) => self.add_textbox(&key, &rect),
                _ => (),
            }
        }
        debug!("Elements: {:?}", &self.elements());
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
    fn add_textbox(&mut self, key: &String, rect: &SvgRect) {
        let id_attrs: Vec<&str> = key.split(':').collect();

        let id = id_attrs
            .get(SvgRectIdAttrs::Id as usize)
            .unwrap_or(&"")
            .to_string();
        let halign = HAlign::from_attr(id_attrs.get(SvgRectIdAttrs::HAlign as usize));
        let valign = VAlign::from_attr(id_attrs.get(SvgRectIdAttrs::VAlign as usize));

        let mut p = Point {
            x: halign.calc_anchor_x(&rect),
            y: valign.calc_anchor_y(&rect),
        };
        self.canvas_transform.transform(&mut p);

        let textbox = TextBox {
            x: p.x,
            y: p.y,
            // TODO: Extract info from attributes/id
            orientation: Orientation::Horizontal,
            visibility: Visibility(true),
            halign: halign,
            valign: valign,
            id: id,
        };
        self.elements.push(Element::TextBox(textbox));
    }

    fn add_symbol_pin(&mut self, key: &str, pinout: &Pinout, mut line: Line) -> Result<()> {
        let id_attrs: Vec<&str> = key.split(':').collect();

        let id = *id_attrs.get(SvgPinIdAttrs::Id as usize).unwrap_or(&"");
        let name_regex = Regex::new(r"^(pin)?\-?(?P<name>.*)$").unwrap();
        let name = name_regex
            .captures(id)
            .unwrap()
            .name("name")
            .unwrap()
            .as_str();

        let halign = HAlign::from_attr(id_attrs.get(SvgPinIdAttrs::HAlign as usize));
        let valign = VAlign::from_attr(id_attrs.get(SvgPinIdAttrs::VAlign as usize));

        line.transform(&self.canvas_transform);

        let pin = pinout
            .get_first(name)
            .ok_or(QedaError::InvalidPinNameInSvg(name.to_string()))?;
        let sym_pin = SymbolPin::new(pin.clone(), halign, valign, &line);
        self.elements.push(Element::SymbolPin(sym_pin));

        Ok(())
    }
}