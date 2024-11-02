use std::fs::File;
use std::path::{Path, PathBuf};

use tera::{Context, Tera};

use crate::model::{Book, Highlight};

/// Uses a [Book] and optionally a [PathBuf] to a custom template file to render the highlights and
/// notes captured whilst reading to a file in the output [Path].
///
/// Will use a default template if a custom template isn't provided.
pub fn render_output(book: &Book, template: &Option<PathBuf>, output_dir: &Path) {
    let mut tera = Tera::default();
    tera.add_raw_template("default", include_str!("templates/default.md"))
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert(
        "highlights",
        &book
            .highlights()
            .values()
            .cloned()
            .collect::<Vec<Highlight>>(),
    );
    ctx.insert("quotes", &book.quotes());

    let output: tera::Result<()>;
    let mut file_path = output_dir.to_path_buf();
    file_path.push(format!("{}. {}.md", book.author(), book.title()));

    let file: File = match File::create(file_path.as_path()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            // dbg!(e);
            ::std::process::exit(1);
        }
    };

    // use default template unless user has provided one
    if let Some(t) = template {
        tera.add_template_file(t, Some("user")).unwrap();
        output = tera.render_to("user", &ctx, file);
    } else {
        output = tera.render_to("default", &ctx, file);
    }

    if let Err(e) = output {
        eprintln!("Parsing error(s): {}", e);
        // dbg!(e);
        ::std::process::exit(1);
    }
}
