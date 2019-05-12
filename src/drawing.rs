pub trait DrawingElement {
}

pub struct Drawing {
    elements: Vec<Box<dyn DrawingElement>>,
}

impl Drawing {
    pub fn new() -> Drawing {
        Drawing {
            elements: Vec::new(),
        }
    }

    pub fn add_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let line = Line {
            x0,
            y0,
            x1,
            y1,
        };
        self.elements.push(Box::new(line));
    }
}

pub struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl DrawingElement for Line {

}