use crate::geometry::*;
use crate::text::*;

bitflags! {
    pub struct PinKind: u16 {
        const UNSPECIFIED    = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const TRISTATE       = 0x0004;
        const PASSIVE        = 0x0008;
        const POWER          = 0x0010;
        const OPEN_COLLECTOR = 0x0020;
        const OPEN_EMITTER   = 0x0040;
        const NOT_CONNECTED  = 0x0080;
    }
}
bitflags! {
    pub struct PinShape: u16 {
        const LINE           = 0x0000;
        const IN             = 0x0001;
        const OUT            = 0x0002;
        const INVERTED       = 0x0004;
        const CLOCK          = 0x0008;
        const LOW            = 0x0010;
        const FALLING_EDGE   = 0x0020;
        const NON_LOGIC      = 0x0040;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PinDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Pin {
    pub pos: Point,
    pub length: f64,
    pub net: String,
    pub number: String,
    pub kind: PinKind,
    pub shape: PinShape,
    pub direction: PinDirection,
    pub visibility: Visibility,
}

impl Pin {
    pub fn new(net: &str, halign: HAlign, valign: VAlign, l: &Line) -> Self {
        let direction = match halign {
            HAlign::Center => match valign {
                VAlign::Top => PinDirection::Down,
                _ => PinDirection::Up,
            },
            HAlign::Left => PinDirection::Right,
            HAlign::Right => PinDirection::Left,
        };

        let x = match direction {
            PinDirection::Down | PinDirection::Up => l.p.0.x,
            PinDirection::Right => l.min_x(),
            PinDirection::Left => l.max_x(),
        };

        let y = match direction {
            PinDirection::Down => l.max_y(),
            PinDirection::Up => l.min_y(),
            PinDirection::Right | PinDirection::Left => l.p.0.y,
        };

        Pin {
            pos: Point { x: x, y: y },
            length: l.len(),
            net: net.to_string(),
            number: "0".to_string(),
            kind: PinKind::UNSPECIFIED,
            shape: PinShape::LINE,
            direction: direction,
            visibility: Visibility::Visible,
        }
    }

    #[inline]
    pub fn number(&self) -> &String {
        &self.number
    }
}
