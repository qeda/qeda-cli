use svgdom::*;

use crate::errors::*;

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
        let svg = svgdom::Document::from_str(svg)?;
        self.set_svg_center(&svg.root())?;
        self.add_svg_node(&svg.root())
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
    fn add_svg_node(&mut self, node: &Node) -> Result<()> {
        if let Some(id) = node.tag_id() {
            match id {
                ElementId::Defs => return Ok(()), // Skip <defs>
                ElementId::Path => {
                    let path_id = node.id();
                    if *path_id == "ch" || *path_id == "cv" { // Skip center marks as they have been already processed
                        return Ok(());
                    } if path_id.starts_with("pin") {
                        self.add_svg_pin(node)?;
                    } else {
                        self.add_svg_path(node)?;
                    }
                },
                _ => (),
            }
        }
        if node.has_children() {
            for child in node.children() {
                self.add_svg_node(&child)?;
            }
        }
        Ok(())
    }

    fn add_svg_pin(&mut self, node: &Node) -> Result<()> {
        println!("<PIN>");
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
                    if let AttributeValue::Length(ref width) = attr.value {
                        dbg!(width.num);
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn add_svg_path(&mut self, node: &Node) -> Result<()> {
        println!("<PATH>");
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
                    if let AttributeValue::Length(ref width) = attr.value {
                        dbg!(width.num);
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn get_svg_line(&self, attrs: &Attributes) -> Result<Line> {
        let mut line = Line::default();
        for attr in attrs.iter() {
            match attr.id().unwrap() { // TODO: Replace `unwrap`
                AttributeId::D => {
                    if let AttributeValue::Path(ref path) = attr.value {
                        for command in path.iter() {
                            match command {
                                PathSegment::MoveTo { abs, x, y } => {
                                    line.points.0.x = *x;
                                    line.points.0.y = *y;
                                    if !abs {
                                        line.points.0.x += self.x;
                                        line.points.0.y += self.y;
                                    }
                                },
                                PathSegment::LineTo { abs, x, y } => {
                                    line.points.1.x = *x;
                                    line.points.1.y = *y;
                                    if !abs {
                                        line.points.1.x += self.x;
                                        line.points.1.y += self.y;
                                    }
                                },
                                PathSegment::HorizontalLineTo { abs, x } => {
                                    line.points.1.x = *x;
                                    line.points.1.y = line.points.0.y;
                                    if !abs {
                                        line.points.1.x += self.x;
                                    }
                                },
                                PathSegment::VerticalLineTo { abs, y } => {
                                    line.points.1.x = line.points.0.x;
                                    line.points.1.y = *y;
                                    if !abs {
                                        line.points.1.y += self.y;
                                    }
                                },
                                _ => (),  
                            }
                        }
                    }
                },
                _ => (),
            }
        }
        Ok(line)
    }

    fn set_svg_center(&mut self, node: &Node) -> Result<()> {
        if let Some(id) = node.tag_id() {
            match id {
                ElementId::Path => {
                    let path_id = node.id();
                    if *path_id == "ch" {
                        let ch = self.get_svg_line(&node.attributes())?;
                        dbg!(ch);
                    } else if *path_id == "cv" {
                        let cv = self.get_svg_line(&node.attributes())?;
                        dbg!(cv);
                    }
                },
                _ => (),
            }
        }
        if node.has_children() {
            for child in node.children() {
                self.set_svg_center(&child)?;
            }
        }
        Ok(())
    }
}