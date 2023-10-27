# whiskers

## Usage

```console
$ whiskers --help
Usage: whiskers [OPTIONS] [TEMPLATE] [FLAVOR]

Arguments:
  [TEMPLATE]  Path to the template file to render, or `-` for stdin
  [FLAVOR]    Flavor to get colors from [possible values: latte, frappe, macchiato, mocha]

Options:
  -l, --list-helpers  List all template helpers in markdown format
  -h, --help          Print help
```

See [the example template](examples/example.hbs) for a starting point, and read on for more details.

## Template Syntax

Templates are written in [Handlebars](https://handlebarsjs.com/guide/expressions.html) syntax.

### Context Variables

The following variables are available for use in your templates:

- `flavor` (string): The name of the flavor being templated. Possible values: `latte`, `frappé`, `macchiato`, `mocha`.
- `isLight` (bool): True if `flavor` is `latte`, false otherwise.
- `isDark` (bool): True unless `flavor` is `latte`.
- All named colors in the flavor, such as `red`, `subtext0`, and `crust`. A full list of named colors can be found [here](https://github.com/catppuccin/rust/blob/5124eb99eb98d7111dca24537d428a6078e5bbb6/src/flavour.rs#L41-L66). Each color is formatted as hex by default.
- All frontmatter variables as described in the [Frontmatter](#frontmatter) section.

### Helpers

The following custom helpers are available:

- `uppercase string` : Convert a string to uppercase.
  - `{{ uppercase "hello" }}` → `HELLO`
- `lowercase string` : Convert a string to lowercase.
  - `{{ lowercase "HELLO" }}` → `hello`
- `titlecase string` : Convert a string to titlecase.
  - `{{ titlecase "hello there" }}` → `Hello There`
- `trunc number places` : Format a number to a string with a given number of places.
  - `{{ trunc 3.14159265 2 }}` → `3.14`
- `lighten color amount` : Lighten a color by a percentage.
  - `{{ lighten red 0.1 }}` → `f8bacc` / `hsl(343, 81%, 85%)`
- `darken color amount` : Darken a color by a percentage.
  - `{{ darken red 0.1 }}` → `ee5c85` / `hsl(343, 81%, 65%)`
- `mix color_a color_b ratio` : Mix two colors together in a given ratio.
  - `{{ mix red base 0.3 }}` → `5e4054` (30% red, 70% base)
- `opacity color amount` : Set the opacity of a color.
  - `{{ opacity red 0.5 }}` → `hsla(343, 81%, 75%, 0.50)`
- `unquote value` : Marks a value to be unquoted. Mostly useful for maintaining JSON syntax highlighting in template files when a non-string value is needed.
  - `{{ unquote isLight true }}` → `true` (the surrounding quotation marks have been removed)
- `rgb color` : Convert a color to CSS RGB format.
  - `{{ rgb red }}` → `rgb(243, 139, 168)`
- `rgba color` : Convert a color to CSS RGBA format.
  - `{{ rgba (opacity red 0.6) }}` → `rgba(243, 139, 168, 0.60)`
- `hsl color` : Convert a color to CSS HSL format.
  - `{{ hsl red }}` → `hsl(343, 81%, 75%)`
- `hsla color` : Convert a color to CSS HSLA format.
  - `{{ hsla (opacity red 0.6) }}` → `hsla(343, 81%, 75%, 0.60)`
- `red_i color` : Get the red channel of a color as an integer from 0 to 255.
  - `{{ red_i red }}` → `243`
- `green_i color` : Get the green channel of a color as an integer from 0 to 255.
  - `{{ green_i red }}` → `139`
- `blue_i color` : Get the blue channel of a color as an integer from 0 to 255.
  - `{{ blue_i red }}` → `168`
- `alpha_i color` : Get the alpha channel of a color as an integer from 0 to 255.
  - `{{ alpha_i (opacity red 0.6) }}` → `153`
- `red_f color` : Get the red channel of a color as a float from 0 to 1.
  - `{{ red_f red }}` → `0.95` (truncated to 2 places)
- `green_f color` : Get the green channel of a color as a float from 0 to 1.
  - `{{ green_f red }}` → `0.55` (truncated to 2 places)
- `blue_f color` : Get the blue channel of a color as a float from 0 to 1.
  - `{{ blue_f red }}` → `0.66` (truncated to 2 places)
- `alpha_f color` : Get the alpha channel of a color as a float from 0 to 1.
  - `{{ alpha_f (opacity red 0.6) }}` → `0.60` (truncated to 2 places)
- `darklight if-dark if-light` : Choose a value depending on the current flavor. Latte is light, while Frappé, Macchiato, and Mocha are all dark.
  - `{{ darklight "Night" "Day" }}` → `Day` on Latte, `Night` on other flavors

## Frontmatter

You can include additional context variables in the templating process by adding it to an optional YAML frontmatter section at the top of your template file.

As a simple example, given the following template (`example.cfg`):

```handlebars
--- app: 'Pepperjack' author: 'winston' --- # Catppuccin for
{{app}}
# by
{{author}}
bg = '{{base}}' fg = '{{text}}'
```

Running `whiskers example.cfg mocha` produces the following output:

```ini
# Catppuccin for Pepperjack
# by winston
bg = '1e1e2e'
fg = 'cdd6f4'
```

Values in YAML frontmatter are rendered in the same way as the rest of the template, which means you can also make use of context variables in your frontmatter. This can be useful for things like setting an accent color:

```handlebars
--- accent: "{{mauve}}" darkGreen: "{{darken green 0.3}}" --- bg = "#{{base}}"
fg = "#{{text}}" border = "#{{accent}}" diffAddFg = "#{{green}}" diffAddBg = "#{{darkGreen}}"
```

Rendering the above template produces the following output:

```ini
bg = "#1e1e2e"
fg = "#cdd6f4"
border = "#cba6f7"
diffaddfg = "#a6e3a1"
diffaddbg = "#40b436"
```

## Overrides

Whiskers supports overriding individual template values without changing the underlying template source. To use this feature, pass the `--override` flag to the whiskers CLI. You can use the `--override` flag multiple times to apply multiple overrides.

We'll use the following template for this example:

```handlebars
--- accent: '{{mauve}}' --- bg = "#{{base}}" fg = "#{{accent}}"
```

With no overrides passed to whiskers, we get the following output:

```ini
bg = "#1e1e2e"
fg = "#cba6f7"
```

However, we can pass overrides through the CLI with `--override accent=40b436`. Then, we get:

```ini
bg = "#1e1e2e"
fg = "#40b436"
```

We can also override with another value from the [template context](context-variables), for example `--override accent=sky`. This gives the following result:

```ini
bg = "#1e1e2e"
fg = "#89dceb"
```

Finally, we can override both values by passing two overrides. If we invoke whiskers with `--override accent=yellow --override base=000000` then we get this output:

```ini
bg = "#000000"
fg = "#f9e2af"
```

## Wishlist

- Combined operation mode, for example setting flavor to `all` and having all four flavors available in the template context.
- Swap out [`css-colors`](https://github.com/vaidehijoshi/css-colors) colour operations for something else (maybe [`farver`](https://github.com/nyxkrage/farver)), ideally with a better colour model.
