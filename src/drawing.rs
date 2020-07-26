use std::collections::HashMap;
use regex::Regex;

use crate::errors::*;
use crate::geometry::*;
use crate::text::*;
use crate::pin::*;
use crate::svg::{self, *};

#[derive(Debug)]
pub enum Element {
    Line(Line),
    TextBox(TextBox),
    Pin(Pin),
}

#[derive(Debug)]
pub struct Drawing {
    canvas_transform: Transformation,
    elements: Vec<Element>,
    attrs: HashMap<String, String>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            canvas_transform: Transformation::new(),
            elements: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn mut_elements(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

    pub fn from_svg(svg: &str) -> Result<Drawing> {
        let mut drawing = Drawing::new();
        drawing.add_svg(svg)?;
        Ok(drawing)
    }

    pub fn add_svg(&mut self, svg: &str) -> Result<()> {
        let mut elements = svg::to_elements(svg)?;
        let mut sx = 1.0;
        let mut sy = 1.0;
        let mut dx = 0.0;
        let mut dy = 0.0;
        if let Some(SvgElement::HLine(ch)) = elements.remove("ch") {
            sx = 1.0/ch.len();
            dx = -ch.cx();
        }
        if let Some(SvgElement::VLine(cv)) = elements.remove("cv") {
            sy = 1.0/cv.len();
            dy = -cv.cy();
        }
        self.canvas_transform.translate(dx, dy);
        // SVG has y axis directed downwards. We need to turn it upwards
        self.canvas_transform.scale(sx, -sy);

        debug!("SVG elements: {:?}", &elements);
        for (key, element) in elements {
            match element {
                SvgElement::HLine(line) => {
                    if key.starts_with("pin") {
                        self.add_pin(&key, line.x0, line.y, line.x1, line.y)
                    } else {
                        self.add_line(line.x0, line.y, line.x1, line.y, line.width)
                    }
                },
                SvgElement::VLine(line) => {
                    if key.starts_with("pin") {
                        self.add_pin(&key, line.x, line.y0, line.x, line.y1)
                    } else {
                        self.add_line(line.x, line.y0, line.x, line.y1, line.width)
                    }
                },
                SvgElement::Rect(rect) => self.add_textbox(&key, &rect),
                _ => ()
            }
        }
        debug!("Elements: {:?}", &self.elements());
        Ok(())
    }

    pub fn add_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, width: f64) {
        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x1, y: y1 };
        let mut line = Line { p: (p0, p1), width };
        line.transform(&self.canvas_transform);
        self.elements.push(Element::Line(line));
    }

    pub fn add_attr(&mut self, key: &str, value: &str) {
        self.attrs.insert(key.to_string(), value.to_string());
    }

    pub fn attr(&self, key: &str, def: &str) -> String {
        self.attrs.get(key).unwrap_or(&def.to_string()).clone()
    }
}

// Private methods
impl Drawing {
    fn add_textbox(&mut self, key: &String, rect: &SvgRect) {
        let id_attrs: Vec<&str> = key.split(':').collect();

        let id = id_attrs.get(SvgRectIdAttrs::Id as usize).unwrap_or(&"").to_string();
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
            // TODO: extract info from attributes/id
            orientation: Orientation::Horizontal,
            visibility: Visibility::Visible,
            halign: halign,
            valign: valign,
            id: id,
        };
        self.elements.push(Element::TextBox(textbox));
    }

    fn add_pin(&mut self, key: &String, x0: f64, y0: f64, x1: f64, y1: f64) {
        let id_attrs: Vec<&str> = key.split(':').collect();

        let id = *id_attrs.get(SvgPinIdAttrs::Id as usize).unwrap_or(&"");
        let net_regex = Regex::new(r"^(pin)?\-?(?P<net>.*)$").unwrap();
        let net = net_regex.captures(id).unwrap().name("net").unwrap().as_str();

        let halign = HAlign::from_attr(id_attrs.get(SvgPinIdAttrs::HAlign as usize));
        let valign = VAlign::from_attr(id_attrs.get(SvgPinIdAttrs::VAlign as usize));

        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x1, y: y1 };
        let mut line = Line { p: (p0, p1), width: 0. };
        line.transform(&self.canvas_transform);

        let pin = Pin::new(&net, halign, valign, &line);
        self.elements.push(Element::Pin(pin));
    }
}
