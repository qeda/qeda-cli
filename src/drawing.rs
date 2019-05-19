use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum  Element {
    Line {
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    },
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

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements 
    }

    pub fn add_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let line = Element::Line {
            x0,
            y0,
            x1,
            y1,
        };
        self.elements.push(line);
    }
}
