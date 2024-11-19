# KTR

The idea of this utility is to convert the highlights I've made in my Kindle to a format that I can use in my Zettelkasten (Obsidian).

## Installation

There are no binaries currently provided via a github release, the only way to install is via `cargo`:

```sh
cargo install --git https://github.com/iambenzo/ktr.git --branch main
```

This will install the `ktr` CLI tool and a `ktr_gui` GUI tool.

> If there's demand, I'll create a release action.

## Usage

This application doesn't do anything too fancy. It takes a path to your 'My Clippings.txt' file, found on your Kindle under the `documents` directory.

You can optionally supply your own [Tera](https://github.com/Keats/tera) template if you want to deviate from the [default output](./kindle_clippings/src/templates/default.md)/structure.

Finally, you need to provide a path to a directory for the output files to land in.

The output is a set of files, one per book, containing your Kindle highlights ready for augmenting into your Zettelkasten. Any Kindle notes attached to a highlight will also be included by the default template.

### CLI

For the CLI, a default `output` directory will be created if one isn't supplied by you, the user.

The CLI will parse your entire clippings file every time.

```sh
Usage: ktr [OPTIONS] <CLIPPINGS_FILE>

Arguments:
  <CLIPPINGS_FILE>

Options:
  -t, --template <TEMPLATE_FILE>
  -o, --output <OUTPUT_DIR>
  -h, --help                      Print help
  -V, --version                   Print version
```

> [!WARNING]
> The error messages aren't pretty.

### GUI

The GUI is a wizard style application.

The benefit of the GUI over the CLI is that it will allow you to select which books are processed into output files...though you will have to use your mouse.

## Templating

For those of you comfortable reading a little Rust, you can take a look at [this file](./kindle_clippings/src/output.rs) to understand what objects are available to your custom template.

For those of you who like tables, here you go:

| Object | Type | Notes |
| ------ | ---- | ----- |
| date | String | Today's date, excluding time |
| highlights | Vec | An iterable list of a book's highlights |
| quotes | Vec | An iterable list of a book's quotes |

If you take a look at the [model](./kindle_clippings/src/model.rs), you'll see that there's opportunity to make more objects available for templating. If there's demand, then I could look to expand the list of available objects for templating.
