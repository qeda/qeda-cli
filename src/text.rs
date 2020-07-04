use crate::svg::SvgRect;

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self { Orientation::Horizontal }
}

#[derive(Debug)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl HAlign {
    pub fn from_attr(attr: Option<&&str>) -> HAlign {
        match attr {
            Some(&"left") => HAlign::Left,
            Some(&"right") => HAlign::Right,
            Some(&"center") => HAlign::Center,
            _ => HAlign::default(),
        }
    }

    pub fn calc_anchor_x(&self, rect: &SvgRect) -> f64 {
        match self {
            HAlign::Left => rect.x,
            HAlign::Center => rect.x + rect.width / 2.,
            HAlign::Right => rect.x + rect.width,
        }
    }
}

impl Default for HAlign {
    fn default() -> Self { HAlign::Left }
}

#[derive(Debug)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

impl VAlign {
    pub fn from_attr(attr: Option<&&str>) -> VAlign {
        match attr {
            Some(&"bottom") => VAlign::Bottom,
            Some(&"top") => VAlign::Top,
            Some(&"center") => VAlign::Center,
            _ => VAlign::default(),
        }
    }

    pub fn calc_anchor_y(&self, rect: &SvgRect) -> f64 {
        match self {
            VAlign::Top => rect.y,
            VAlign::Center => rect.y + rect.height / 2.,
            VAlign::Bottom => rect.y + rect.height,
        }
    }
}

impl Default for VAlign {
    fn default() -> Self { VAlign::Center }
}

#[derive(Debug)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Default for Visibility {
    fn default() -> Self { Visibility::Visible }
}

#[derive(Default, Debug)]
pub struct TextBox {
    pub x: f64,
    pub y: f64,
    pub orientation: Orientation,
    pub visibility: Visibility,
    pub halign: HAlign,
    pub valign: VAlign,
    pub id: String,
}
