use std::fmt;

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntoEnumIterator, IntEnum, PartialEq)]

pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    let _color: ResistorColor = match ResistorColor::from_int(value as u8) {
        Ok(value) => value,
        _ => return String::from("value out of range"),
    };
    _color.to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
