#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here
mod frontmatter;
mod helper;
mod parse;
mod postprocess;
mod template;

use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::{eyre::Context, Result};

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

    let mut ctx = template::make_context(args.flavor.into())?;
    let (content, frontmatter) = frontmatter::render_and_parse(&tpl, &reg, &ctx);
    if let Some(frontmatter) = frontmatter {
        ctx.as_object_mut().expect("ctx is an object value").extend(
            frontmatter
                .as_object()
                .expect("frontmatter is an object value")
                .clone(),
        );
    }

    reg.register_template_string(template_name, content)
        .wrap_err("Failed to parse template")?;
    let result = reg
        .render(template_name, &ctx)
        .wrap_err("Failed to render template")?;
    let result = postprocess(&result);
    println!("{result}");

    Ok(())
}

