#![allow(unused_macros)]
#![allow(dead_code)]

use std::fmt;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

macro_rules! warn {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::warn_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}
macro_rules! err {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::err_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}
macro_rules! errln {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::errln_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}
macro_rules! info {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::info_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}
macro_rules! infoln {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::infoln_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}
macro_rules! debug {
    ( $ ( $ arg : tt ) * ) => ( $crate::log::debug_fmt ( format_args ! ( $ ( $ arg ) * ) ) )
}

pub async fn warn_async(message: &str) {
    text_async("warning: ", Color::Yellow, message).await;
}

pub fn warn_fmt(args: fmt::Arguments<'_>) {
    text_fmt("warning: ", Color::Yellow, args);
}

pub fn err_fmt(args: fmt::Arguments<'_>) {
    text_fmt("error: ", Color::Red, args);
}

pub fn errln_fmt(args: fmt::Arguments<'_>) {
    text_fmt("|      ", Color::Red, args);
}

pub async fn info_async(message: &str) {
    text_async("info: ", Color::Green, message).await;
}

pub fn info_fmt(args: fmt::Arguments<'_>) {
    text_fmt("info: ", Color::Green, args);
}

pub fn infoln_fmt(args: fmt::Arguments<'_>) {
    text_fmt("|      ", Color::Green, args);
}

pub fn debug_fmt(args: fmt::Arguments<'_>) {
    if std::env::var("QEDA_DEBUG").is_ok() {
        text_fmt("debug: ", Color::Blue, args);
    }
}

fn text_fmt(preamble: &str, color: Color, args: fmt::Arguments<'_>) {
    let mut t = StandardStream::stderr(ColorChoice::Auto);
    let mut color_spec = ColorSpec::new();
    let color_spec = color_spec
        .set_fg(Some(color))
        .set_intense(true)
        .set_bold(true);
    let _ = t.set_color(color_spec);
    let _ = write!(t, "{}", preamble);
    let _ = t.reset();
    let _ = t.write_fmt(args);
    let _ = writeln!(t);
}

async fn text_async(preamble: &str, color: Color, text: &str) {
    let mut t = StandardStream::stderr(ColorChoice::Auto);
    let mut color_spec = ColorSpec::new();
    let color_spec = color_spec
        .set_fg(Some(color))
        .set_intense(true)
        .set_bold(true);
    let _ = t.set_color(color_spec);
    let _ = write!(t, "{}", preamble);
    let _ = t.reset();
    let _ = t.write_all(text.as_bytes());
    let _ = writeln!(t);
}
