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
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            canvas_transform: Transformation::new(),
            elements: Vec::new(),
        }
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
        self.canvas_transform.scale(sx, sy);
        self.canvas_transform.translate(dx, dy);

        dbg!(&elements);
        for (_, element) in elements {
            match element {
                SvgElement::HLine(line) => self.add_line(line.x0, line.y, line.x1, line.y),
                SvgElement::VLine(line) => self.add_line(line.x, line.y0, line.x, line.y1),
                _ => ()
            }
        }
        dbg!(&self.elements());
        Ok(())
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements 
    }

    pub fn add_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let mut p0 = Point { x: x0, y: y0 };
        self.canvas_transform.transform(&mut p0);
        let mut p1 = Point { x: x1, y: y1 };
        self.canvas_transform.transform(&mut p1);
        let line = Line { p: (p0, p1) };
        self.elements.push(Element::Line(line));
    }
}

// Private methods
impl Drawing {
    
}
