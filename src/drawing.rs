use nalgebra::{Point2, Transform2};

use crate::errors::*;
use crate::svg::{self, *};

type TransformMatrix = Transform2<f64>;

pub trait Transform {
    fn transform(&mut self, matrix: &TransformMatrix);
}

#[derive(Debug)]
pub enum Element {
    Line(Line),
}

#[derive(Default, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Transform for Point {
    fn transform(&mut self, matrix: &TransformMatrix) {
        let p = Point2::new(self.x, self.y);
        let p = matrix.transform_point(&p);
        self.x = p.x;
        self.y = p.y;
    }
}

#[derive(Default, Debug)]
pub struct Line {
    pub p: (Point, Point)
}

impl Transform for Line {
    fn transform(&mut self, matrix: &TransformMatrix) {
        self.p.0.transform(matrix);
        self.p.1.transform(matrix);
    }
}

#[derive(Debug)]
pub struct Drawing {
    canvas_transform: TransformMatrix,
    elements: Vec<Element>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            canvas_transform: TransformMatrix::identity(),
            elements: Vec::new(),
        }
    }

    pub fn translate_canvas(&mut self, dx: f64, dy: f64) {
        // TODO: Modify canvas_transform
    }

    pub fn rotate_canvas(&mut self, angle: f64) {
        // TODO: Modify canvas_transform
    }

    pub fn scale_canvas(&mut self, sx: f64, sy: f64) {
        // TODO: Modify canvas_transform
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
        let line = Line { p: (p0, p1) };
        self.elements.push(Element::Line(line));
    }
}

// Private methods
impl Drawing {
    
}
