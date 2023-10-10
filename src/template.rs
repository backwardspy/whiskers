use std::collections::HashMap;

use color_eyre::{eyre::eyre, Result};
use css_colors::Color;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::helper;

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorData {
    name: String,
    hsl: String,
    hsla: String,
    rgb: String,
    rgba: String,
    hex: String,
    hexa: String,
    r: u8,
    g: u8,
    b: u8,
}

pub fn make_registry() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    reg.register_helper("uppercase", Box::new(helper::uppercase));
    reg.register_helper("lowercase", Box::new(helper::lowercase));
    reg.register_helper("titlecase", Box::new(helper::titlecase));
    reg.register_helper("lighten", Box::new(helper::lighten));
    reg.register_helper("darken", Box::new(helper::darken));
    reg.register_helper("mix", Box::new(helper::mix));
    reg.register_helper("opacity", Box::new(helper::opacity));
    reg.register_helper("darklight", Box::new(helper::darklight));
    reg.set_strict_mode(true);
    reg
}

pub fn make_context(flavor: catppuccin::Flavour) -> Result<serde_json::Value> {
    let colors = flavor.colours();

    let color_map: HashMap<String, ColorData> = colors
        .into_fields_iter()
        .map(|(name, c)| {
            let rgb: css_colors::RGB = c.into();
            let hsl: css_colors::HSL = rgb.to_hsl();
            (
                name.to_string(),
                ColorData {
                    name: name.to_string(),
                    hsl: hsl.to_string(),
                    hsla: hsl.to_hsla().to_string(),
                    rgb: rgb.to_string(),
                    rgba: rgb.to_rgba().to_string(),
                    hex: c.hex(),
                    hexa: format!("{}FF", c.hex()),
                    r: c.0,
                    g: c.1,
                    b: c.2,
                },
            )
        })
        .collect();

    let mut context = serde_json::to_value(color_map)?;

    context["flavor"] = flavor.name().into();
    context["isLight"] = (flavor == catppuccin::Flavour::Latte).into();

    Ok(context)
}

pub fn merge_user_context<S>(
    context: serde_json::Value,
    user_context: S,
) -> Result<serde_json::Value>
where
    S: Serialize,
{
    let mut ctx = serde_json::to_value(context)?
        .as_object_mut()
        .ok_or_else(|| eyre!("Internal error: context is not an object"))?
        .clone();

    let user_ctx = serde_json::to_value(user_context)?
        .as_object()
        .ok_or_else(|| eyre!("YAML frontmatter must be a hash at the top-level"))?
        .clone();

    ctx.extend(user_ctx);

    Ok(serde_json::Value::Object(ctx))
}
