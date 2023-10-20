use base64::Engine;
use css_colors::{Color, Ratio};
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperResult, Output, RenderContext,
    RenderError,
};

use ::titlecase::titlecase as titlecase_ext;
use serde_json::Value;

use crate::{format, parse};

fn format_color_string_with<F>(color_string: &str, formatter: F) -> String
where
    F: FnOnce(css_colors::HSLA, format::ColorModel) -> String,
{
    color_string.parse::<parse::Color>().map_or_else(
        |err| {
            eprintln!("Warning: Failed to parse color string '{color_string}': {err}");
            color_string.to_string()
        },
        |result| match result {
            parse::Color::Hsl(hsl) => formatter(hsl.to_hsla(), format::ColorModel::Hsl),
            parse::Color::Hsla(hsla) => formatter(hsla, format::ColorModel::Hsla),
            parse::Color::Rgb(rgb) => formatter(rgb.to_hsla(), format::ColorModel::Rgb),
            parse::Color::Rgba(rgba) => formatter(rgba.to_hsla(), format::ColorModel::Rgba),
            parse::Color::Hex(rgb) => formatter(rgb.to_hsla(), format::ColorModel::Hex),
            parse::Color::Hexa(rgba) => formatter(rgba.to_hsla(), format::ColorModel::Hexa),
        },
    )
}

fn format_color_string<F>(color_string: &str, formatter: F) -> String
where
    F: FnOnce(css_colors::HSLA) -> css_colors::HSLA,
{
    format_color_string_with(color_string, |hsla, format| {
        format::color(formatter(hsla), format)
    })
}

handlebars_helper!(uppercase: |s: String| s.to_uppercase());
handlebars_helper!(lowercase: |s: String| s.to_lowercase());
handlebars_helper!(titlecase: |s: String| titlecase_ext(&s));
handlebars_helper!(lighten: |color: String, weight: f32| {
    format_color_string(&color, |hsl| hsl.lighten(Ratio::from_f32(weight)))
});
handlebars_helper!(darken: |color: String, weight: f32| {
    format_color_string(&color, |hsl| hsl.darken(Ratio::from_f32(weight)))
});
handlebars_helper!(mix: |a: String, b: String, t: f32| {
    format_color_string_with(&a, |a_hsla, format| {
        format_color_string_with(&b, |b_hsla, _| {
            format::color(a_hsla.mix(b_hsla, Ratio::from_f32(t)), format)
        })
    })
});
handlebars_helper!(opacity: |color: String, amount: f32| {
    format_color_string(&color, |hsl| hsl.fade(Ratio::from_f32(amount)))
});

pub fn darklight(
    h: &Helper,
    _r: &Handlebars,
    ctx: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let dark = h
        .param(0)
        .ok_or_else(|| RenderError::new("Missing parameter `dark` in position 0"))?;
    let light = h
        .param(1)
        .ok_or_else(|| RenderError::new("Missing parameter `light` in position 1"))?;

    if ctx.data()["flavor"] == "latte" {
        out.write(&light.render())?;
    } else {
        out.write(&dark.render())?;
    }

    Ok(())
}

handlebars_helper!(unquote: |value: Value| {
    let content = serde_json::to_string(&value)?;
    let content = base64::engine::general_purpose::STANDARD_NO_PAD.encode(content);
    format!("{{WHISKERS:UNQUOTE:{content}}}")
});
