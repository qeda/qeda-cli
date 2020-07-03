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
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl HorizontalAlignment {
    pub fn from_attr(attr: Option<&&str>) -> HorizontalAlignment {
        match attr {
            Some(&"left") => HorizontalAlignment::Left,
            Some(&"right") => HorizontalAlignment::Right,
            Some(&"center") => HorizontalAlignment::Center,
            _ => HorizontalAlignment::default(),
        }
    }

    pub fn calc_anchor_x(&self, rect: &SvgRect) -> f64 {
        match self {
            HorizontalAlignment::Left => rect.x,
            HorizontalAlignment::Center => rect.x + rect.width / 2.,
            HorizontalAlignment::Right => rect.x + rect.width,
        }
    }
}

impl Default for HorizontalAlignment {
    fn default() -> Self { HorizontalAlignment::Left }
}

#[derive(Debug)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl VerticalAlignment {
    pub fn from_attr(attr: Option<&&str>) -> VerticalAlignment {
        match attr {
            Some(&"bottom") => VerticalAlignment::Bottom,
            Some(&"top") => VerticalAlignment::Top,
            Some(&"center") => VerticalAlignment::Center,
            _ => VerticalAlignment::default(),
        }
    }

    pub fn calc_anchor_y(&self, rect: &SvgRect) -> f64 {
        match self {
            VerticalAlignment::Top => rect.y,
            VerticalAlignment::Center => rect.y + rect.height / 2.,
            VerticalAlignment::Bottom => rect.y + rect.height,
        }
    }
}

impl Default for VerticalAlignment {
    fn default() -> Self { VerticalAlignment::Center }
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
    pub halign: HorizontalAlignment,
    pub valign: VerticalAlignment,
    pub id: String,
}
