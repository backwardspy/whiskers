use std::collections::HashMap;

use color_eyre::Result;
use handlebars::Handlebars;

use crate::helper;

pub fn make_registry() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    reg.register_helper("uppercase", Box::new(helper::uppercase));
    reg.register_helper("lowercase", Box::new(helper::lowercase));
    reg.register_helper("titlecase", Box::new(helper::titlecase));
    reg.register_helper("lighten", Box::new(helper::lighten));
    reg.register_helper("darken", Box::new(helper::darken));
    reg.register_helper("mix", Box::new(helper::mix));
    reg.register_helper("opacity", Box::new(helper::opacity));
    reg.register_helper("unquote", Box::new(helper::unquote));
    reg.register_helper("rgb", Box::new(helper::rgb));
    reg.register_helper("rgba", Box::new(helper::rgba));
    reg.register_helper("hsl", Box::new(helper::hsl));
    reg.register_helper("hsla", Box::new(helper::hsla));
    reg.register_helper("red_i", Box::new(helper::red_i));
    reg.register_helper("green_i", Box::new(helper::green_i));
    reg.register_helper("blue_i", Box::new(helper::blue_i));
    reg.register_helper("alpha_i", Box::new(helper::alpha_i));
    reg.register_helper("red_f", Box::new(helper::red_f));
    reg.register_helper("green_f", Box::new(helper::green_f));
    reg.register_helper("blue_f", Box::new(helper::blue_f));
    reg.register_helper("alpha_f", Box::new(helper::alpha_f));
    reg.register_helper("darklight", Box::new(helper::darklight));
    reg.set_strict_mode(true);
    reg
}

pub fn make_context(flavor: catppuccin::Flavour) -> Result<serde_json::Value> {
    let colors = flavor.colours();

    let color_map: HashMap<String, String> = colors
        .into_fields_iter()
        .map(|(name, c)| (name.to_string(), c.hex().to_ascii_lowercase()))
        .collect();

    let mut context = serde_json::to_value(color_map)?;

    context["flavor"] = flavor.name().into();
    context["isLight"] = (flavor == catppuccin::Flavour::Latte).into();
    context["isDark"] = (flavor != catppuccin::Flavour::Latte).into();

    Ok(context)
}
