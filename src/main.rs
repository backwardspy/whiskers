#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here
use std::{fs, path::PathBuf};

use clap::Parser;
use color_eyre::{eyre::Context, Result};

use whiskers::frontmatter;
use whiskers::postprocess::postprocess;
use whiskers::template::{self, helpers};

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
    #[arg(required_unless_present = "list_helpers")]
    template_path: Option<PathBuf>,

    #[arg(value_enum, required_unless_present = "list_helpers")]
    flavor: Option<Flavor>,

    #[arg(short, long)]
    list_helpers: bool,
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .panic_section("Consider reporting this issue: https://github.com/catppuccin/toolbox")
        .display_env_section(false)
        .install()?;

    let args = Args::parse();

    if args.list_helpers {
        list_helpers();
        return Ok(());
    }

    let template_path = &args
        .template_path
        .expect("template_path is guaranteed to be set");
    let flavor = args.flavor.expect("flavor is guaranteed to be set");

    let tpl = fs::read_to_string(template_path).wrap_err(format!(
        "Failed to read template file '{}'",
        template_path.display()
    ))?;

    let mut reg = template::make_registry();

    let template_name = template_path
        .file_name()
        .and_then(|p| p.to_str())
        .unwrap_or("unnamed template");

    let mut ctx = template::make_context(flavor.into());
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

fn list_helpers() {
    for helper in helpers() {
        print!("- `{}", helper.name);
        for arg in helper.args {
            print!(" {arg}");
        }
        println!("` : {}", helper.description);
        for (before, after) in helper.examples {
            println!("    - `{{{{ {} {} }}}}` → {}", helper.name, before, after);
        }
    }
}

