use super::prelude::*;

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
