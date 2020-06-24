use linked_hash_map::LinkedHashMap;
use svgdom::*;

use crate::errors::*;

#[derive(Clone, Default, Debug)]
pub struct SvgPoint {
    pub x: f64,
    pub y: f64,
    pub marker: bool,
}

#[derive(Debug)]
pub struct SvgLine {
    pub p: (SvgPoint, SvgPoint),
    pub width: f64,
}

#[derive(Default, Debug)]
pub struct SvgHLine {
    pub x0: f64,
    pub x1: f64,
    pub y: f64,
    pub width: f64,
}

impl SvgHLine {
    pub fn cx(&self) -> f64 {
        (self.x0 + self.x1) / 2.0
    }

    pub fn len(&self) -> f64 {
        (self.x1 - self.x0).abs()
    }
}

#[derive(Default, Debug)]
pub struct SvgVLine {
    pub x: f64,
    pub y0: f64,
    pub y1: f64,
    pub width: f64,
}

impl SvgVLine {
    pub fn cy(&self) -> f64 {
        (self.y0 + self.y1) / 2.0
    }

    pub fn len(&self) -> f64 {
        (self.y1 - self.y0).abs()
    }
}

#[derive(Default, Debug)]
pub struct SvgPolygon {
    pub p: Vec<SvgPoint>,
    pub width: f64,
}

#[derive(Debug)]
pub enum SvgElement {
    HLine(SvgHLine),
    VLine(SvgVLine),
    Line(SvgLine),
    Polygon(SvgPolygon),
}

pub type SvgHash = LinkedHashMap<String, SvgElement>;

#[derive(Default, Debug)]
struct Svg {
    elements: SvgHash,
    current_x: f64,
    current_y: f64,
}

impl Svg {
    fn new() -> Self {
        Self::default()
    }

    fn add_node(&mut self, node: &Node) -> Result<()> {
        if let Some(id) = node.tag_id() {
            match id {
                ElementId::Defs => return Ok(()), // Skip <defs>
                ElementId::Path => {
                    let path_id = node.id().to_string();
                    let polygon = self.to_polygon(&node.attributes())?;
                    if polygon.p.len() == 2 {
                        if polygon.p[0].y == polygon.p[1].y {
                            let line = SvgHLine {
                                x0: polygon.p[0].x,
                                x1: polygon.p[1].x,
                                y: polygon.p[0].y,
                                width: polygon.width,
                            };
                            self.elements.insert(path_id, SvgElement::HLine(line));
                        } else if polygon.p[0].x == polygon.p[1].x {
                            let line = SvgVLine {
                                x: polygon.p[0].x,
                                y0: polygon.p[0].y,
                                y1: polygon.p[1].y,
                                width: polygon.width,
                            };
                            self.elements.insert(path_id, SvgElement::VLine(line));
                        } else {
                            let line = SvgLine {
                                p: (polygon.p[0].clone(), polygon.p[1].clone()),
                                width: polygon.width,
                            };
                            self.elements.insert(path_id, SvgElement::Line(line));
                        }
                    } else {
                        self.elements.insert(path_id, SvgElement::Polygon(polygon));
                    }
                },
                _ => (),
            }
        }
        if node.has_children() {
            for child in node.children() {
                self.add_node(&child)?;
            }
        }
        Ok(())
    }

    fn length_to_coordinate(length: &Length) -> f64 {
        match length.unit {
            LengthUnit::None => length.num,
            _ => panic!("Unexpected length unit: {:?}", length.unit),
        }
    }

    fn to_polygon(&mut self, attributes: &Attributes) -> Result<SvgPolygon> {
        let mut polygon = SvgPolygon::default();
        for attr in attributes.iter() {
                match attr.id().ok_or(ErrorKind::InvalidSvgPath)? {
                    AttributeId::D => {
                        if let AttributeValue::Path(ref path) = attr.value {
                            for command in path.iter() {
                                match command {
                                    PathSegment::MoveTo { abs, x, y } |
                                    PathSegment::LineTo { abs, x, y } => {
                                        let mut x = *x;
                                        let mut y = *y;
                                        if !abs {
                                            x += self.current_x;
                                            y += self.current_y;
                                        }
                                        self.current_x = x;
                                        self.current_y = y;
                                        polygon.p.push(SvgPoint { x, y, marker: false });
                                    },
                                    PathSegment::HorizontalLineTo { abs, x } => {
                                        let mut x = *x;
                                        let y = self.current_y;
                                        if !abs {
                                            x += self.current_x;
                                        }
                                        self.current_x = x;
                                        polygon.p.push(SvgPoint { x, y, marker: false });
                                    },
                                    PathSegment::VerticalLineTo { abs, y } => {
                                        let x = self.current_x;
                                        let mut y = *y;
                                        if !abs {
                                            y += self.current_y;
                                        }
                                        self.current_y = y;
                                        polygon.p.push(SvgPoint { x, y, marker: false });
                                    },
                                    _ => (),
                                }
                            }
                        }
                    },
                    AttributeId::StrokeWidth => {
                        if let AttributeValue::Length(ref width) = attr.value {
                            polygon.width = Svg::length_to_coordinate(&width);
                        }
                    },
                    _ => (),
                }
            }
        Ok(polygon)
    }
}

pub fn to_elements(svg: &str) -> Result<SvgHash> {
    let svg_doc = svgdom::Document::from_str(svg)?;
    let mut svg = Svg::new();
    svg.add_node(&svg_doc.root())?;
    Ok(svg.elements)
}
