use super::prelude::*;
use super::{Line, Point, Transform, Transformation};

use crate::pinout::Pin;

#[derive(Debug)]
pub struct SymbolPin {
    pub pin: Pin,
    pub origin: Point,
    pub length: f64,
    pub direction: PinDirection,
    pub visibility: Visibility,
}

impl SymbolPin {
    /// Creates an empty `SymbolPin`.
    pub fn new(pin: Pin, halign: HAlign, valign: VAlign, l: &Line) -> Self {
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

        SymbolPin {
            pin,
            origin: Point::new(x, y),
            length: l.length(),
            direction,
            visibility: Visibility::default(),
        }
    }
}

impl Transform for SymbolPin {
    fn transform(mut self, t: &Transformation) -> Self {
        self.origin = self.origin.transform(t);
        self.length *= t.scale;
        // TODO: Change `direction` according to rotation
        self
    }
}
