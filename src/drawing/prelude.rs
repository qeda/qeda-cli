#[derive(Debug)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl HAlign {
    pub fn from_str(s: &str) -> Self {
        match s {
            "left" => HAlign::Left,
            "right" => HAlign::Right,
            "center" => HAlign::Center,
            _ => HAlign::default(),
        }
    }
}

impl Default for HAlign {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug)]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}

impl VAlign {
    pub fn from_str(s: &str) -> Self {
        match s {
            "bottom" => VAlign::Bottom,
            "top" => VAlign::Top,
            "middle" => VAlign::Middle,
            _ => VAlign::default(),
        }
    }
}

impl Default for VAlign {
    fn default() -> Self {
        Self::Bottom
    }
}

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Horizontal
    }
}

#[derive(Debug, Default)]
pub struct Visibility(pub bool);

#[derive(Debug)]
pub enum PinDirection {
    Up,
    Down,
    Right,
    Left,
}
