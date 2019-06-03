use crate::errors::*;
use crate::svg::{self, *};

#[derive(Debug)]
pub enum  Element {
    Line(Line),
}

#[derive(Default, Debug)]
pub struct Line {
    pub points: (Point, Point)
}

#[derive(Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Drawing {
    x: f64,
    y: f64,
    elements: Vec<Element>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            x: 0.0,
            y: 0.0,
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
        if let Some(SvgElement::HLine(ch)) = elements.remove("ch") {
            dbg!(ch);
        }
        if let Some(SvgElement::VLine(cv)) = elements.remove("cv") {
            dbg!(cv);
        }
        dbg!(&elements);
        Ok(())
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements 
    }

    pub fn add_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x1, y: y1 };
        let line = Line { points: (p0, p1) };
        self.elements.push(Element::Line(line));
    }
}

// Private methods
impl Drawing {
    
}
