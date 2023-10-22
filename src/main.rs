#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
#![allow(clippy::cast_possible_truncation)] // we like truncating u32s into u8s around here
use clap::Parser;
use clap_stdin::FileOrStdin;
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
    /// Path to the template file to render, or `-` for stdin
    #[arg(required_unless_present = "list_helpers")]
    template: Option<FileOrStdin>,

    /// Flavor to get colors from
    #[arg(value_enum, required_unless_present = "list_helpers")]
    flavor: Option<Flavor>,

    /// List all template helpers in markdown format
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

    let template = &args
        .template
        .expect("template_path is guaranteed to be set");

    let flavor = args.flavor.expect("flavor is guaranteed to be set");

    let reg = template::make_registry();

    let mut ctx = template::make_context(flavor.into());
    let (content, frontmatter) = frontmatter::render_and_parse(template, &reg, &ctx);
    if let Some(frontmatter) = frontmatter {
        ctx.as_object_mut().expect("ctx is an object value").extend(
            frontmatter
                .as_object()
                .expect("frontmatter is an object value")
                .clone(),
        );
    }

    let result = reg
        .render_template(content, &ctx)
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

