use crate::config::Config;
use crate::drawing::{Drawing, Layer, Line, Pad, Rect};

pub fn draw_body(drawing: &mut Drawing, body: &Rect, _pads: &[Pad], lib_cfg: &Config) {
    let line_width = lib_cfg.get_f64("pattern.line-width.silkscreen").unwrap();

    let rect = body
        .clone()
        .expand(line_width / 2.0)
        .line_width(line_width)
        .layer(Layer::SILKSCREEN_TOP);

    let lines: Vec<Line> = rect.to_lines();
    drawing.add_lines(lines);
}
