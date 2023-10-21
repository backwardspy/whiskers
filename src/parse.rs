use std::str::FromStr;

use color_eyre::{
    eyre::{eyre, ErrReport},
    Result,
};
use css_colors::{Color as CssColor, Ratio};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while_m_n},
    character::complete::{i32, multispace0, u8},
    combinator::map_res,
    multi::many_m_n,
    number::complete::float,
    sequence::{delimited, preceded, terminated, Tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq)]
pub enum Color {
    Hsl(css_colors::HSL),
    Hsla(css_colors::HSLA),
    Rgb(css_colors::RGB),
    Rgba(css_colors::RGBA),
    Hex(css_colors::RGB),
    Hexa(css_colors::RGBA),
}

fn percentage(s: &str) -> IResult<&str, u8> {
    terminated(u8, tag("%"))(s)
}

fn sep(s: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag(","), multispace0)(s)
}

fn hex_pair(s: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()), |s| {
        u8::from_str_radix(s, 16)
    })
    .parse(s)
}

fn args(s: &str) -> IResult<&str, &str> {
    delimited(tag("("), take_until(")"), tag(")"))(s)
}

impl FromStr for Color {
    type Err = ErrReport;

    fn from_str(s: &str) -> Result<Self> {
        let (_, result) = alt((
            Self::parse_hsla,
            Self::parse_hsl,
            Self::parse_rgba,
            Self::parse_rgb,
            Self::parse_hexa,
            Self::parse_hex,
        ))(s)
        .map_err(|e| eyre!(e.to_owned()))?;
        Ok(result)
    }
}

impl Color {
    fn parse_hsla(s: &str) -> IResult<&str, Self> {
        let (s, inner) = preceded(tag("hsla"), args)(s)?;
        let (_, (hue, _, sat, _, light, _, alpha)) =
            (i32, sep, percentage, sep, percentage, sep, float).parse(inner)?;
        Ok((s, Self::Hsla(css_colors::hsla(hue, sat, light, alpha))))
    }

    fn parse_hsl(s: &str) -> IResult<&str, Self> {
        let (s, inner) = preceded(tag("hsl"), args)(s)?;
        let (_, (hue, _, sat, _, light)) = (i32, sep, percentage, sep, percentage).parse(inner)?;
        Ok((s, Self::Hsl(css_colors::hsl(hue, sat, light))))
    }

    fn parse_rgba(s: &str) -> IResult<&str, Self> {
        let (s, inner) = preceded(tag("rgba"), args)(s)?;
        let (_, (red, _, green, _, blue, _, alpha)) =
            (u8, sep, u8, sep, u8, sep, float).parse(inner)?;
        Ok((s, Self::Rgba(css_colors::rgba(red, green, blue, alpha))))
    }

    fn parse_rgb(s: &str) -> IResult<&str, Self> {
        let (s, inner) = preceded(tag("rgb"), args)(s)?;
        let (_, (red, _, green, _, blue)) = (u8, sep, u8, sep, u8).parse(inner)?;
        Ok((s, Self::Rgb(css_colors::rgb(red, green, blue))))
    }

    fn parse_hex(s: &str) -> IResult<&str, Self> {
        let (s, _) = many_m_n(0, 1, tag("#"))(s)?;
        let (s, (red, green, blue)) = (hex_pair, hex_pair, hex_pair).parse(s)?;
        Ok((s, Self::Hex(css_colors::rgb(red, green, blue))))
    }

    fn parse_hexa(s: &str) -> IResult<&str, Self> {
        let (s, _) = many_m_n(0, 1, tag("#"))(s)?;
        let (s, (red, green, blue, alpha)) = (hex_pair, hex_pair, hex_pair, hex_pair).parse(s)?;
        Ok((
            s,
            Self::Hexa(css_colors::rgba(
                red,
                green,
                blue,
                Ratio::from_u8(alpha).as_f32(),
            )),
        ))
    }

    fn to_rgb(&self) -> css_colors::RGB {
        match self {
            Self::Hsl(hsl) => hsl.to_rgb(),
            Self::Hsla(hsla) => hsla.to_rgb(),
            Self::Rgb(rgb) | Self::Hex(rgb) => *rgb,
            Self::Rgba(rgba) | Self::Hexa(rgba) => rgba.to_rgb(),
        }
    }

    pub fn red(&self) -> u8 {
        self.to_rgb().r.as_u8()
    }

    pub fn green(&self) -> u8 {
        self.to_rgb().g.as_u8()
    }

    pub fn blue(&self) -> u8 {
        self.to_rgb().b.as_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hsl() -> Result<()> {
        let result: Color = "hsl(120, 50%, 50%)".parse()?;
        assert_eq!(result, Color::Hsl(css_colors::hsl(120, 50, 50)));
        Ok(())
    }

    #[test]
    fn parse_hsla() -> Result<()> {
        let result: Color = "hsla(120, 50%, 50%, 0.5)".parse()?;
        assert_eq!(result, Color::Hsla(css_colors::hsla(120, 50, 50, 0.5)));
        Ok(())
    }

    #[test]
    fn parse_rgb() -> Result<()> {
        let result: Color = "rgb(123, 56, 78)".parse()?;
        assert_eq!(result, Color::Rgb(css_colors::rgb(123, 56, 78)));
        Ok(())
    }

    #[test]
    fn parse_rgba() -> Result<()> {
        let result: Color = "rgba(123, 56, 78, 0.5)".parse()?;
        assert_eq!(result, Color::Rgba(css_colors::rgba(123, 56, 78, 0.5)));
        Ok(())
    }

    #[test]
    fn parse_hex() -> Result<()> {
        let result: Color = "#94e2d5".parse()?;
        assert_eq!(result, Color::Hex(css_colors::rgb(148, 226, 213)));
        Ok(())
    }

    #[test]
    fn parse_hex_no_hash() -> Result<()> {
        let result: Color = "94e2d5".parse()?;
        assert_eq!(result, Color::Hex(css_colors::rgb(148, 226, 213)));
        Ok(())
    }

    #[test]
    fn parse_hexa() -> Result<()> {
        let result: Color = "#94e2d580".parse()?;
        assert_eq!(result, Color::Hexa(css_colors::rgba(148, 226, 213, 0.5)));
        Ok(())
    }

    #[test]
    fn parse_hexa_no_hash() -> Result<()> {
        let result: Color = "94e2d580".parse()?;
        assert_eq!(result, Color::Hexa(css_colors::rgba(148, 226, 213, 0.5)));
        Ok(())
    }
}
