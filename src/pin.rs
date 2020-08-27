use crate::geometry::*;
use crate::text::*;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ElectricKind {
    Input,
    Output,
    Bidirectional,
    Tristate,
    Passive,
    Unspecified,
    PowerInput,
    PowerOutput,
    OpenCollector,
    OpenEmitter,
    NotConnected,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum PinShape {
    Line,
    Inverted,
    Clock,
    InvertedClock,
    InputLow,
    ClockLow,
    OutputLow,
    FallingEdgeClock,
    NonLogic,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum PinOrientation {
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
    pub ekind: ElectricKind,
    pub shape: PinShape,
    pub orientation: PinOrientation,
    pub visibility: Visibility,
}

impl Pin {
    pub fn new(net: &str, halign: HAlign, valign: VAlign, l: &Line) -> Self {
        let orientation = match halign {
            HAlign::Center => match valign {
                VAlign::Top => PinOrientation::Down,
                _ => PinOrientation::Up,
            },
            HAlign::Left => PinOrientation::Right,
            HAlign::Right => PinOrientation::Left,
        };

        let posx = match orientation {
            PinOrientation::Down | PinOrientation::Up => l.p.0.x,
            PinOrientation::Right => l.p.0.x.min(l.p.1.x),
            PinOrientation::Left => l.p.0.x.max(l.p.1.x),
        };

        let posy = match orientation {
            PinOrientation::Down => l.p.0.y.max(l.p.1.y),
            PinOrientation::Up => l.p.0.y.min(l.p.1.y),
            PinOrientation::Right | PinOrientation::Left => l.p.0.y,
        };

        Pin {
            pos: Point { x: posx, y: posy },
            length: l.length(),
            net: net.to_string(),
            number: "0".to_string(),
            ekind: ElectricKind::Unspecified,
            shape: PinShape::Line,
            orientation: orientation,
            visibility: Visibility::Visible,
        }
    }
}
