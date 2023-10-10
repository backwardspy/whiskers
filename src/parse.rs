use std::str::FromStr;

use color_eyre::{eyre::eyre, Result};
use css_colors::Ratio;
use regex::Regex;

pub enum Color {
    Hsl(css_colors::HSL),
    Hsla(css_colors::HSLA),
    Rgb(css_colors::RGB),
    Rgba(css_colors::RGBA),
    Hex(css_colors::RGB),
    Hexa(css_colors::RGBA),
}

struct X32(u32);

impl FromStr for X32 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(s, 16).map(Self)
    }
}

macro_rules! parser {
    ( $ctor:ident -> Color::$variant:ident, $pattern:expr, $( $component:ident: $type:ty),+ => $make_css:expr) => {
        fn $ctor(c: &str) -> Result<Option<Color>> {
            let pattern = Regex::new($pattern)?;

            if let Some(caps) = pattern.captures(c) {
                let (_, [$( $component ),+]) = caps.extract();
                $( let $component: $type = $component.parse()?; )+
                Ok(Some(Color::$variant($make_css)))
            } else {
                Ok(None)
            }
        }
    };

    ( $ctor:ident -> Color::$variant:ident, $pattern:expr, $( $component:ident: $type:ty ),+ ) => {
        parser!($ctor -> Color::$variant, $pattern, $( $component: $type ),+ => css_colors::$ctor($($component),+));
    };
}

parser!(
    hsl -> Color::Hsl,
    r"^hsl\((\d+),\s*(\d+)%,\s*(\d+)%\)$",
    hue: i32, saturation: u8, lightness: u8
);
parser!(
    hsla -> Color::Hsla,
    r"^hsla\((\d+),\s*(\d+)%,\s*(\d+)%,\s*(\d+(?:\.\d+)?)\)$",
    hue: i32, saturation: u8, lightness: u8, alpha: f32
);
parser!(
    rgb -> Color::Rgb,
    r"^rgb\((\d+),\s*(\d+),\s*(\d+)\)$",
    red: u8, green: u8, blue: u8
);
parser!(
    rgba -> Color::Rgba,
    r"^rgba\((\d+),\s*(\d+),\s*(\d+),\s*(\d+(?:\.\d+)?)\)$",
    red: u8, green: u8, blue: u8, alpha: f32
);
parser!(
    hex -> Color::Hex,
    r"^#?([0-9a-fA-F]{6})$",
    i: X32 => css_colors::rgb((i.0 >> 16) as u8, (i.0 >> 8) as u8, i.0 as u8)
);
parser!(
    hexa -> Color::Hexa,
    r"^#?([0-9a-fA-F]{8})$",
    i: X32 => css_colors::rgba(
        (i.0 >> 24) as u8,
        (i.0 >> 16) as u8,
        (i.0 >> 8) as u8,
        Ratio::from_u8(i.0 as u8).as_f32(),
    )
);

pub fn color(value: &str) -> Result<Color> {
    if let Some(hsl) = hsl(value)? {
        Ok(hsl)
    } else if let Some(hsla) = hsla(value)? {
        Ok(hsla)
    } else if let Some(rgb) = rgb(value)? {
        Ok(rgb)
    } else if let Some(rgba) = rgba(value)? {
        Ok(rgba)
    } else if let Some(hex) = hex(value)? {
        Ok(hex)
    } else if let Some(hexa) = hexa(value)? {
        Ok(hexa)
    } else {
        Err(eyre!("Failed to match color string {value} to any parser."))
    }
}
