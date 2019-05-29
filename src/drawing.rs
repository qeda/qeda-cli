use svgdom::*;

use crate::errors::*;

#[derive(Debug)]
pub enum  Element {
    Line(Line),
}

#[derive(Debug)]
pub struct Line {
    pub points: (Point, Point)
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Drawing {
    elements: Vec<Element>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            elements: Vec::new(),
        }
    }

    pub fn from_svg(svg: &str) -> Result<Drawing> {
        let mut drawing = Drawing::new();
        let svg = svgdom::Document::from_str(svg)?;
        for (id, node) in svg.root().descendants().svg() {
            match id {
                ElementId::Path => {
                    println!("path");
                    let path_id = node.id();
                    dbg!(path_id);
                    for attr in node.attributes().iter() {
                        match attr.id().unwrap() {
                            AttributeId::D => {
                                if let AttributeValue::Path(ref path) = attr.value {
                                    for i in path.iter() {
                                        dbg!(i);
                                    }
                                }
                            },
                            AttributeId::StrokeWidth => {
                                dbg!(&attr.value);
                            },
                            _ => {},
                        }
                    }
                },
                _ => {}
            }   
        }
        Ok(drawing)
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements 
    }

    pub fn add_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let p0 = Point { x: x0, y: y0 };
        let p1 = Point { x: x1, y: y1 };
        let line = Line { points: (p0, p1) };
        self.elements.push(Element::Line(line));
    }
}

// Private methods
impl Drawing {

}