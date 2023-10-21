#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here

mod format;
mod helper;
mod parse;
mod postprocess;
mod template;

use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use color_eyre::{eyre::Context, Result};
use handlebars::Handlebars;
use serde_json::Value;
use yaml_front_matter::YamlFrontMatter;

use postprocess::postprocess;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<Flavor> for catppuccin::Flavour {
    fn from(value: Flavor) -> Self {
        match value {
            Flavor::Latte => Self::Latte,
            Flavor::Frappe => Self::Frappe,
            Flavor::Macchiato => Self::Macchiato,
            Flavor::Mocha => Self::Mocha,
        }
    }
}

#[derive(clap::Parser, Debug)]
struct Args {
    template_path: PathBuf,

    #[arg(value_enum)]
    flavor: Flavor,
}

type Frontmatter = HashMap<String, String>;

fn try_get_frontmatter(template: &str) -> (String, Option<Frontmatter>) {
    match YamlFrontMatter::parse::<Frontmatter>(template) {
        Ok(doc) => (doc.content, Some(doc.metadata)),
        Err(e) => {
            eprintln!("Template has no valid frontmatter ({e}).");
            (template.to_string(), None)
        }
    }
}

fn render_frontmatter(frontmatter: Frontmatter, ctx: &Value, reg: &Handlebars) -> Frontmatter {
    frontmatter
        .into_iter()
        .map(|(k, v)| {
            (
                k.clone(),
                reg.render_template(&v, &ctx).unwrap_or_else(|e| {
                    eprintln!("Warning: failed to render template for frontmatter key '{k}': {e}");
                    v
                }),
            )
        })
        .collect::<Frontmatter>()
}

fn make_context(
    flavor: Flavor,
    reg: &Handlebars,
    frontmatter: Option<Frontmatter>,
) -> Result<Value> {
    let mut ctx = template::make_context(flavor.into())?;

    // render frontmatter values as templates
    if let Some(frontmatter) = frontmatter {
        let user_ctx = render_frontmatter(frontmatter, &ctx, reg);
        ctx = template::merge_user_context(ctx, user_ctx)?;
    }

    Ok(ctx)
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue: https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = Args::parse();
    let tpl = fs::read_to_string(&args.template_path).wrap_err(format!(
        "Failed to read template file '{}'",
        args.template_path.display()
    ))?;

    let mut reg = template::make_registry();

    let template_name = args
        .template_path
        .file_name()
        .and_then(|p| p.to_str())
        .unwrap_or("unnamed template");

    let (content, frontmatter) = try_get_frontmatter(&tpl);
    let ctx = make_context(args.flavor, &reg, frontmatter)?;
    reg.register_template_string(template_name, content)
        .wrap_err("Failed to parse template")?;
    let result = reg
        .render(template_name, &ctx)
        .wrap_err("Failed to render template")?;
    let result = postprocess(&result);
    println!("{result}");

    Ok(())
}
