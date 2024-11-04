use std::fs::File;
use std::path::{Path, PathBuf};

use chrono::Utc;
use tera::{Context, Tera};

use crate::model::{Book, Highlight};

fn now_date() -> String {
    format!("{}", Utc::now().format("%Y-%m-%d"))
}

/// Uses a [Book] and optionally a [PathBuf] to a custom template file to render the highlights and
/// notes captured whilst reading to a file in the output [Path].
///
/// Will use a default template if a custom template isn't provided.
pub fn render_output(
    book: &Book,
    template: &Option<PathBuf>,
    output_dir: &Path,
) -> Result<(), RenderError> {
    let mut tera = Tera::default();
    tera.add_raw_template("default", include_str!("templates/default.md"))
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("date", &now_date());
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
            return Err(RenderError::CreateOutputFileFailed(e.to_string()));
            // eprintln!("Error creating output file: {}", e);
            // ::std::process::exit(1);
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
        return Err(RenderError::ParsingFailed(e.to_string()));
        // eprintln!("Parsing error(s): {}", e);
        // ::std::process::exit(1);
    }

    Ok(())
}

pub enum RenderError {
    CreateOutputFileFailed(String),
    ParsingFailed(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::CreateOutputFileFailed(s) => write!(f, "{}", s),
            RenderError::ParsingFailed(s) => write!(f, "{}", s),
        }
    }
}
