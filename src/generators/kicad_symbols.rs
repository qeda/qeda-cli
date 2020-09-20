use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use crate::component::Component;
use crate::config::Config;
use crate::drawing::*;
use crate::error::*;
use crate::pinout::*;
use crate::symbol::Symbol;

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
            VAlign::Middle => write!(f, "C"),
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
            PinKind::PASSIVE => write!(f, "P"),
            PinKind::POWER => write!(f, "W"),
            PinKind::OPEN_COLLECTOR => write!(f, "C"),
            PinKind::OPEN_EMITTER => write!(f, "E"),
            PinKind::NOT_CONNECTED => write!(f, "N"),
            x if x == (PinKind::IN | PinKind::OUT) => write!(f, "B"),
            x if x == (PinKind::POWER | PinKind::IN) => write!(f, "W"),
            x if x == (PinKind::POWER | PinKind::OUT) => write!(f, "w"),
            x if x.contains(PinKind::HI_Z) => write!(f, "T"),
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
            PinShape::NON_LOGIC | PinShape::ANALOG => write!(f, "X"),
            x if x == (PinShape::CLOCK | PinShape::INVERTED) => write!(f, "CI"),
            x if x == (PinShape::IN | PinShape::ACTIVE_LOW) => write!(f, "L"),
            x if x == (PinShape::CLOCK | PinShape::ACTIVE_LOW) => write!(f, "CL"),
            x if x == (PinShape::OUT | PinShape::ACTIVE_LOW) => write!(f, "V"),
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

#[derive(Debug, Default)]
pub struct KicadSymbols {
    name: String,
    font_size_name: i64,
    font_size_pin: i64,
    font_size_ref_des: f64,
    font_size_value: f64,
    space_pin: i64,
}

impl KicadSymbols {
    pub fn new(name: &str) -> Self {
        KicadSymbols {
            name: name.to_string(),
            ..Self::default()
        }
    }

    /// Renders symbols to a KiCad symbol library.
    pub fn render(self, components: &[Component]) -> Result<()> {
        let mut f = File::create(format!("{}.lib", self.name))?;
        writeln!(f, "EESchema-LIBRARY Version 2.4")?;
        writeln!(f, "#encoding utf-8")?;

        for component in components {
            let name = &component.name;
            let symbol = &component.symbol;
            ensure!(
                !symbol.parts.is_empty(),
                QedaError::InvalidSymbolNoParts(name.to_string())
            );
            info!("  • symbol: '{}'", name);

            // Header
            writeln!(f, "{}", self.header(name, symbol))?;

            // Fields
            let first_part = symbol.parts.first().unwrap();
            if let Some(ref_des) = first_part.find_attribute("ref-des") {
                let mut ref_des = ref_des.clone();
                ref_des.font_size = self.font_size_ref_des;
                writeln!(f, "{}", self.field(0, &ref_des))?;
            }
            if let Some(value) = first_part.find_attribute("value") {
                let mut value = value.clone();
                value.value = name.clone();
                value.font_size = self.font_size_value;
                writeln!(f, "{}", self.field(1, &value))?;
            }

            // Parts
            for (number, part) in symbol.parts.iter().enumerate() {
                writeln!(f, "DRAW")?;
                for element in &part.elements {
                    if let Some(element) = self.element(number, element) {
                        writeln!(f, "{}", element)?;
                    }
                }
                writeln!(f, "ENDDRAW")?;
            }
            writeln!(f, "ENDDEF")?;
        }

        writeln!(f, "#\n#End Library")?;
        Ok(())
    }

    /// Builds an `KicadSymbols` with applied settings from `Config`.
    pub fn settings(mut self, lib_cfg: &Config) -> Self {
        let unit = lib_cfg.get_f64("generator.symbol.unit").unwrap();

        self.font_size_name =
            (unit * lib_cfg.get_f64("symbol.font-size.name").unwrap()).round() as i64;
        self.font_size_pin =
            (unit * lib_cfg.get_f64("symbol.font-size.pin").unwrap()).round() as i64;
        self.font_size_ref_des =
            (unit * lib_cfg.get_f64("symbol.font-size.ref-des").unwrap()).round();
        self.font_size_value = (unit * lib_cfg.get_f64("symbol.font-size.value").unwrap()).round();
        self.space_pin = (unit * lib_cfg.get_f64("symbol.space.pin").unwrap()).round() as i64;

        self
    }

    // Render element to a library file record
    fn element(&self, number: usize, element: &Element) -> Option<String> {
        match element {
            Element::Line(l) => Some(format!(
                "P {points_number} {unit} {convert} {thickness} {x1} {y1} {x2} {y2} N",
                points_number = 2,
                unit = number,
                convert = 1, // 0 if common to the 2 representations, if not 1 or 2
                thickness = l.width.round(),
                x1 = l.p.0.x.round(),
                y1 = l.p.0.y.round(),
                x2 = l.p.1.x.round(),
                y2 = l.p.1.y.round(),
            )),
            Element::SymbolPin(sym_pin) => Some(format!(
                "X {name} {number} {posx} {posy} {length} {orientation} {snum} {snom} \
                {unit} {convert} {etype} {visibility}{shape}",
                name = sym_pin.pin.name,
                number = sym_pin.pin.number,
                posx = sym_pin.origin.x.round(),
                posy = sym_pin.origin.y.round(),
                length = sym_pin.length.round(),
                orientation = sym_pin.direction,
                snum = self.font_size_pin,  // pin number text size
                snom = self.font_size_name, // pin name text size
                unit = number, // 0 if common to all parts. If not, number of the part (1. .n)
                convert = 1,   // 0 if common to the representations, if not 1 or 2
                etype = sym_pin.pin.kind,
                visibility = match sym_pin.visibility {
                    Visibility(true) => "",
                    Visibility(false) => "N",
                },
                shape = sym_pin.pin.shape,
            )),
            _ => None,
        }
    }

    // Render field to a library file record
    fn field(&self, number: i64, attr: &Attribute) -> String {
        format!(
            "F{number} \"{text}\" {x} {y} {dimension} {orientation} {visibility} {hjustify} {vjustify}NN",
            number = number,
            text = attr.value,
            x = attr.origin.x,
            y = attr.origin.y,
            dimension = attr.font_size,
            orientation = attr.orientation,
            visibility = attr.visibility,
            hjustify = attr.halign,
            vjustify = attr.valign,
        )
    }

    // Render a library file header
    fn header(&self, name: &str, symbol: &Symbol) -> String {
        format!(
            "#\n# {name}\n#\n\
            DEF {name} {ref_des} 0 {text_offset} {draw_pinnumber} {draw_pinname} {unit_count} {units_locked} {option_flag}",
            name = name,
            ref_des = symbol.ref_des,
            text_offset = self.space_pin,
            draw_pinnumber = if symbol.show_pin_numbers { "Y" } else { "N" },
            draw_pinname = if symbol.show_pin_names { "Y" } else { "N" },
            unit_count = symbol.parts.len(),
            units_locked = "L",
            option_flag = if symbol.power { "P" } else { "N" },
        )
    }
}
