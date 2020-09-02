use std::fmt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::config::Config;
use crate::drawing::*;
use crate::error::*;
use crate::generators::GeneratorHandler;
use crate::geometry::Transform;
use crate::library::Library;
use crate::pinout::*;

const KICADLIB_DIR: &str = "kicadlib";

pub struct KicadGenerator {}

impl KicadGenerator {
    pub fn new() -> KicadGenerator {
        KicadGenerator {}
    }
}

impl GeneratorHandler for KicadGenerator {
    fn render(&self, name: &str, library: &Library) -> Result<()> {
        info!("rendering KiCad symbol library: '{}.lib'", name);
        fs::create_dir_all(KICADLIB_DIR)?;
        self.render_symbols(name, library)?;

        info!("rendering KiCad pattern library: '{}.pretty'", name);
        fs::create_dir_all(format!("{}/{}.pretty", KICADLIB_DIR, name))?;

        info!("rendering KiCad 3D library: '{}.3dshapes'", name);
        fs::create_dir_all(format!("{}/{}.3dshapes", KICADLIB_DIR, name))?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum FieldKind {
    Reference = 0,
    Value = 1,
}

struct FontSize {
    pub default: f64,
    pub ref_des: f64,
    pub name: f64,
    pub pin: f64,
}

impl FontSize {
    pub fn new(config: &Config) -> Result<FontSize> {
        Ok(FontSize {
            default: config.get_f64("generator.font-size.default")?,
            ref_des: config.get_f64("generator.font-size.ref-des")?,
            name: config.get_f64("generator.font-size.name")?,
            pin: config.get_f64("generator.font-size.pin")?,
        })
    }

    pub fn size(&self, kind: FieldKind) -> f64 {
        match kind {
            FieldKind::Reference => self.ref_des,
            FieldKind::Value => self.name,
        }
    }
}

struct GeneratorParameters {
    grid: f64,
    font_size: FontSize,
}

impl GeneratorParameters {
    pub fn new(config: &Config) -> Result<GeneratorParameters> {
        Ok(GeneratorParameters {
            grid: config.get_f64("generator.symbol-grid")?,
            font_size: FontSize::new(&config)?,
        })
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "H"),
            Orientation::Vertical => write!(f, "V"),
        }
    }
}

impl fmt::Display for HAlign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HAlign::Left => write!(f, "L"),
            HAlign::Center => write!(f, "C"),
            HAlign::Right => write!(f, "R"),
        }
    }
}

impl fmt::Display for VAlign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VAlign::Top => write!(f, "T"),
            VAlign::Center => write!(f, "C"),
            VAlign::Bottom => write!(f, "B"),
        }
    }
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Visibility(true) => write!(f, "V"),
            Visibility(false) => write!(f, "H"),
        }
    }
}

impl fmt::Display for PinKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PinKind::UNSPECIFIED => write!(f, "U"),
            PinKind::IN => write!(f, "I"),
            PinKind::OUT => write!(f, "O"),
            x if x == (PinKind::IN | PinKind::OUT) => write!(f, "B"),
            PinKind::TRISTATE => write!(f, "T"),
            PinKind::PASSIVE => write!(f, "P"),
            PinKind::POWER => write!(f, "W"),
            x if x == (PinKind::POWER | PinKind::IN) => write!(f, "W"),
            x if x == (PinKind::POWER | PinKind::OUT) => write!(f, "w"),
            PinKind::OPEN_COLLECTOR => write!(f, "C"),
            PinKind::OPEN_EMITTER => write!(f, "E"),
            PinKind::NOT_CONNECTED => write!(f, "N"),
            _ => write!(f, "U"),
        }
    }
}

impl fmt::Display for PinShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PinShape::LINE => write!(f, ""),
            PinShape::INVERTED => write!(f, "I"),
            PinShape::CLOCK => write!(f, "C"),
            x if x == (PinShape::INVERTED | PinShape::CLOCK) => write!(f, "CI"),
            PinShape::IN | PinShape::LOW => write!(f, "L"),
            x if x == (PinShape::CLOCK | PinShape::LOW) => write!(f, "CL"),
            x if x == (PinShape::OUT | PinShape::LOW) => write!(f, "V"),
            PinShape::FALLING_EDGE => write!(f, "F"),
            x if x == (PinShape::FALLING_EDGE | PinShape::CLOCK) => write!(f, "F"),
            PinShape::NON_LOGIC => write!(f, "X"),
            _ => write!(f, ""),
        }
    }
}

impl fmt::Display for PinDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PinDirection::Up => write!(f, "U"),
            PinDirection::Down => write!(f, "D"),
            PinDirection::Right => write!(f, "R"),
            PinDirection::Left => write!(f, "L"),
        }
    }
}

impl KicadGenerator {
    fn render_symbols(&self, name: &str, library: &Library) -> Result<()> {
        let params = GeneratorParameters::new(&library.config())?;
        let mut f = File::create(format!("{}/{}.lib", KICADLIB_DIR, name))?;

        f.write(b"EESchema-LIBRARY Version 2.4\n")?;
        f.write(b"#encoding utf-8\n")?;

        let components = library.components();
        for component in components {
            let symbol = component.symbol();
            let ref_des = symbol.attr("ref-des", "U");
            KicadGenerator::write_component_header(
                &mut f,
                &ref_des.as_str(),
                &component.name().as_str(),
                &symbol,
            )?;

            let elements = symbol.elements();
            let text_boxes = elements.iter().filter_map(|element| match element {
                Element::TextBox(text_box) => Some(text_box),
                _ => None,
            });
            for text_box in text_boxes {
                KicadGenerator::write_field(
                    &mut f,
                    &ref_des.as_str(),
                    &component.name().as_str(),
                    &params,
                    &text_box,
                )?;
            }

            f.write(b"DRAW\n")?;
            for element in elements.iter() {
                KicadGenerator::write_element(&mut f, &params, &element)?;
            }
            f.write(b"ENDDRAW\n")?;
            f.write(b"ENDDEF\n")?;
        }

        f.write(b"#\n#End Library\n")?;
        Ok(())
    }

    fn write_element(mut f: &File, params: &GeneratorParameters, element: &Element) -> Result<()> {
        match element {
            Element::SymbolPin(sym_pin) => {
                let mut p = sym_pin.origin.clone();
                p.scale(params.grid, params.grid);
                write!(
                    f,
                    "X {name} {number} {posx} {posy} {length} {orientation} {snum} {snom} \
                    {unit} {convert} {etype} {visibility}{shape}\n",
                    name = sym_pin.pin.name,
                    number = sym_pin.pin.number,
                    posx = p.x.round(),
                    posy = p.y.round(),
                    length = (sym_pin.len * params.grid).round(),
                    orientation = sym_pin.direction,
                    snum = params.font_size.pin,  // pin number text size
                    snom = params.font_size.name, // pin name text size
                    unit = 0, // 0 if common to all parts. If not, number of the part (1. .n)
                    convert = 0, // 0 if common to the representations, if not 1 or 2
                    etype = sym_pin.pin.kind,
                    visibility = match sym_pin.visibility {
                        Visibility(true) => "",
                        Visibility(false) => "N",
                    },
                    shape = sym_pin.pin.shape,
                )?;
                debug!(
                    "SymbolPin: {}, {}, ({}, {})",
                    sym_pin.pin.name, sym_pin.pin.number, sym_pin.origin.x, sym_pin.origin.y
                );
            }
            Element::Line(l) => {
                let mut l = l.clone();
                l.scale(params.grid, params.grid);
                write!(
                    f,
                    "P {points_number} {unit} {convert} {thickness} {x1} {y1} {x2} {y2} N\n",
                    points_number = 2,
                    unit = 0, // 0 if common to the parts; if not, number of part
                    // TODO: Replace "unit" by attribute
                    convert = 1, // 0 if common to the 2 representations, if not 1 or 2
                    thickness = l.width.round(),
                    x1 = l.p.0.x.round(),
                    y1 = l.p.0.y.round(),
                    x2 = l.p.1.x.round(),
                    y2 = l.p.1.y.round(),
                )?;
                debug!("Line: {}, {}, {}, {}", l.p.0.x, l.p.0.y, l.p.1.x, l.p.1.y);
            }
            _ => {}
        }
        Ok(())
    }

    fn write_component_header(
        mut f: &File,
        ref_des: &str,
        name: &str,
        symbol: &Drawing,
    ) -> Result<()> {
        write!(f, "#\n# {}\n#\n", name)?;
        write!(
            f,
            "DEF {name} {reference} {unused} {text_offset} {draw_pinnumber} {draw_pinname} \
             {unit_count} {units_locked} {option_flag}\n",
            name = name,
            reference = ref_des,
            unused = 0,      // Required by specification to be zero
            text_offset = 5, // Space. TODO: Replace by attribute
            draw_pinnumber = symbol.attr("show_pin_numbers", "N"),
            draw_pinname = symbol.attr("show_pin_names", "N"),
            unit_count = 1, // Symbols count. TODO: Replace by attribute
            units_locked = "L",
            option_flag = symbol.attr("power", "N"),
        )?;

        Ok(())
    }

    fn write_field(
        mut f: &File,
        ref_des: &str,
        component_name: &str,
        params: &GeneratorParameters,
        text_box: &TextBox,
    ) -> Result<()> {
        let field_kind = match text_box.id.as_str() {
            "ref-des" => Some(FieldKind::Reference),
            "value" => Some(FieldKind::Value),
            _ => None,
        };
        if let Some(field_kind) = field_kind {
            let text = match field_kind {
                FieldKind::Reference => ref_des,
                FieldKind::Value => component_name,
            };
            write!(
                f,
                "F{field_number} \"{text}\" {x} {y} {dimension} {orientation} {visibility} \
                {hjustify} {vjustify}NN\n",
                field_number = field_kind as u8,
                text = text,
                x = (text_box.x * params.grid).round(),
                y = (text_box.y * params.grid).round(),
                dimension = params.font_size.size(field_kind).round(),
                orientation = text_box.orientation,
                visibility = text_box.visibility,
                hjustify = text_box.halign,
                vjustify = text_box.valign,
            )?;
        }

        Ok(())
    }
}
