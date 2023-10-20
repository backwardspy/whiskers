# whiskers

> [!WARNING]
> This tool is a work-in-progress. I would not recommend using it yet as things may change.

Soothing port creation tool for the high-spirited!

## Usage

```console
$ whiskers --help
Usage: whiskers <TEMPLATE_PATH> <FLAVOR>

Arguments:
  <TEMPLATE_PATH>  
  <FLAVOR>      [possible values: latte, frappe, macchiato, mocha]

Options:
  -h, --help  Print help
```

See [the example template](examples/example.template) for a starting point, and read on for more details.

## Template Syntax

Templates are written in [Handlebars](https://handlebarsjs.com/guide/expressions.html) syntax.

### Context Variables

The following variables are available for use in your templates:

- `flavor` (string): The name of the flavor being templated. Possible values: `latte`, `frappé`, `macchiato`, `mocha`.
- `isLight` (bool): True if `flavor` is `latte`, false otherwise.
- `isDark` (bool): True unless `flavor` is `latte`.
- All named colors in the flavor, such as `red`, `subtext0`, and `crust`. A full list of named colors can be found [here](https://github.com/catppuccin/rust/blob/5124eb99eb98d7111dca24537d428a6078e5bbb6/src/flavour.rs#L41-L66). Each color has the following properties:
    - `name` (string): The name of the color.
    - `hsl` (string): The color in CSS HSL format (`hsl(343, 81%, 75%)`)
    - `hsla` (string): The color in CSS HSLA format (`hsla(343, 81%, 75%, 1.0)`)
    - `rgb` (string): The color in CSS RGB format (`rgb(243, 139, 168)`)
    - `rgba` (string): The color in CSS RGBA format (`rgba(243, 139, 168, 1.0)`)
    - `hex` (string): The color in 6-digit hexadecimal format (`#F38BA8`)
    - `hexa` (string): The color in 8-digit hexadecimal format (`#F38BA8FF`)
    - `r` (int): The red component of the color (0-255).
    - `g` (int): The green component of the color (0-255).
    - `b` (int): The blue component of the color (0-255).
- All frontmatter variables as described in the [Frontmatter](#frontmatter) section.

### Helpers

The following custom helpers are available:

- `uppercase string` : Converts a string to uppercase.
    - `{{ uppercase "hello" }}` → `HELLO`
- `lowercase string` : Converts a string to lowercase.
    - `{{ lowercase "HELLO" }}` → `hello`
- `titlecase string` : Converts a string to titlecase.
    - `{{ titlecase "hello there" }}` → `Hello There`
- `lighten color amount` : Lightens a color by a percentage.
    - `{{ lighten red.hex 0.1 }}` → `F8BACC`
    - `{{ lighten red.hsl 0.1 }}` → `hsl(343, 81%, 85%)`
- `darken color amount` : Darkens a color by a percentage.
    - `{{ darken red.hex 0.1 }}` → `EE5C85`
    - `{{ darken red.hsl 0.1 }}` → `hsl(343, 81%, 65%)`
- `mix color_a color_b ratio` : Mixes two colors together with a given ratio.
    - `{{ mix red.hex base.hex 0.3 }}` → `5E4054` (30% red, 70% base)
- `opacity color amount` : Sets a color's opacity.
    - `{{ opacity red.hsla 0.5 }}` → `hsla(343, 81%, 75%, 0.50)`
- `unquote value`: Marks a value to be unquoted. Mostly useful for maintaining JSON syntax highlighting in template files when a non-string value is needed.
    - `"{{ unquote isLight }}"` → `true` (the surrounding quote marks have been removed)
- `darklight dark light` : Chooses a value depending on the set flavor. Latte is light, while Frappé, Macchiato, and Mocha are all dark.
    - `{{ darklight "Night", "Day" }}` → `Day` on Latte, `Night` on other flavors.

## Frontmatter

You can include additional context variables in the templating process by adding it to an optional YAML frontmatter section at the top of your template file.

As a simple example, given the following template (`example.cfg`):

```handlebars
---
app: 'Pepperjack'
author: 'winston'
---
# Catppuccin for {{ app }}
# by {{ author }}
bg = '{{ base.hex }}'
fg = '{{ text.hex }}'
```

Running `whiskers example.cfg mocha` produces the following output:

```ini
# Catppuccin for Pepperjack
# by winston
bg = '1E1E2E'
fg = 'CDD6F4'
```

Values in YAML frontmatter are rendered in the same way as the rest of the template, which means you can also make use of context variables in your frontmatter. This can be useful for things like setting an accent color:

```handlebars
---
accent: '#{{ mauve.hex }}'
darkGreen: '#{{ darken green.hex 0.3 }}'
---
bg = "#{{ base.hex }}"
fg = "#{{ text.hex }}"
border = "{{ accent }}"
diffAddFg = "#{{ green.hex }}"
diffAddBg = "{{ darkGreen }}"
```

Rendering the above template produces the following output:

```ini
bg = "#1E1E2E"
fg = "#CDD6F4"
border = "#CBA6F7"
diffAddFg = "#A6E3A1"
diffAddBg = "#40B436"
```

## Opinions

Any and all feedback is appreciated, especially on the following topics:

- Frontmatter support
- Available helpers for common port creation needs

## Wishlist

- Color overrides option for CLI
    - e.g. `--overrides '{"base": "#000000"}'`
- Combined operation mode, for example setting flavor to `all` and having all four flavors available in the template context.
- Swap out [`css-colors`](https://github.com/vaidehijoshi/css-colors) colour operations for something else (maybe [`farver`](https://github.com/nyxkrage/farver)), ideally with a better colour model.