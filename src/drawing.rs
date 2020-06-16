use std::collections::HashMap;

use crate::errors::*;
use crate::geometry::*;
use crate::svg::{self, *};

#[derive(Debug)]
pub enum Element {
    Line(Line),
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

        dbg!(&elements);
        for (_, element) in elements {
            match element {
                SvgElement::HLine(line) => self.add_line(line.x0, line.y, line.x1, line.y, line.width),
                SvgElement::VLine(line) => self.add_line(line.x, line.y0, line.x, line.y1, line.width),
                _ => ()
            }
        }
        dbg!(&self.elements());
        Ok(())
    }

    pub fn add_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, width: f64) {
        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x1, y: y1 };
        let mut line = Line { p: (p0, p1), width };
        line.transform(&self.canvas_transform);
        self.elements.push(Element::Line(line));
    }

    pub fn add_attr(&mut self, key: &str, value: String) {
        self.attrs.insert(key.to_string(), value);
    }

    pub fn attr(&self, key: &str, def: &str) -> String {
        self.attrs.get(key).unwrap_or(&def.to_string()).clone()
    }
}

// Private methods
impl Drawing {

}
