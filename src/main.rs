#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here

mod format;
mod helper;
mod parse;
mod postprocess;
mod template;

use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use color_eyre::{eyre::eyre, Result};
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

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue: https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = Args::parse();
    let tpl = fs::read_to_string(args.template_path)?;

    let document = YamlFrontMatter::parse::<Frontmatter>(&tpl)
        .map_err(|e| eyre!("Failed to parse YAML front matter: {}", e))?;

    let reg = template::make_registry();
    let ctx = template::make_context(args.flavor.into())?;

    // render frontmatter values as templates
    let user_ctx: Frontmatter = document
        .metadata
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
        .collect();
    let ctx = template::merge_user_context(ctx, user_ctx)?;

    let result = reg.render_template(&document.content, &ctx)?;
    let result = postprocess(&result)?;
    println!("{result}");

    Ok(())
}
