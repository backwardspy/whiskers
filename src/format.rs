use css_colors::Color;

#[derive(Clone, Copy)]
pub enum ColorModel {
    Hsl,
    Hsla,
    Rgb,
    Rgba,
    Hex,
    Hexa,
}

pub fn color(hsla: css_colors::HSLA, model: ColorModel) -> String {
    match model {
        ColorModel::Hsl => hsla.to_hsl().to_string(),
        ColorModel::Hsla => hsla.to_string(),
        ColorModel::Rgb => hsla.to_rgb().to_string(),
        ColorModel::Rgba => hsla.to_rgba().to_string(),
        ColorModel::Hex => {
            let rgb = hsla.to_rgb();
            format!(
                "{:02X}{:02X}{:02X}",
                rgb.r.as_u8(),
                rgb.g.as_u8(),
                rgb.b.as_u8()
            )
        }
        ColorModel::Hexa => {
            let rgba = hsla.to_rgba();
            format!(
                "{:02X}{:02X}{:02X}{:02X}",
                rgba.r.as_u8(),
                rgba.g.as_u8(),
                rgba.b.as_u8(),
                rgba.a.as_u8()
            )
        }
    }
}
