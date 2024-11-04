use clap::Parser;
use kindle_clippings::output::render_output;
use kindle_clippings::{parse_clippings, read_file_string};
use std::env;
use std::fs::create_dir;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[arg(value_name = "CLIPPINGS_FILE")]
    file: PathBuf,

    #[arg(short, long, value_name = "TEMPLATE_FILE")]
    template: Option<PathBuf>,

    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // create/validate provided output directory
    if let Some(o) = cli.output {
        // create directory if it doesn't exist
        if !o.exists() {
            if let Err(e) = create_dir(&o) {
                eprintln!("Unable to create output directory: {}", e);
                ::std::process::exit(1);
            }

            // check provided dir is actually a directory
        } else if !o.is_dir() {
            eprintln!("{} is not a directory!", o.display());
            ::std::process::exit(1);
        }
        run(cli.file, cli.template, &o);

        // default to "output" directory
    } else {
        let mut pwd = env::current_dir().unwrap();
        pwd.push("output");

        if let Err(e) = create_dir(&pwd) {
            eprintln!("Unable to create output directory: {}", e);
            ::std::process::exit(1);
        }

        run(cli.file, cli.template, &pwd);
    }
}

pub fn run(clippings: PathBuf, template: Option<PathBuf>, output_dir: &Path) {
    if let Ok(s) = read_file_string(clippings) {
        let books = parse_clippings(s);

        for (_, book) in books.iter() {
            render_output(book, &template, output_dir);
        }
    } else {
        panic!("error reading file");
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
