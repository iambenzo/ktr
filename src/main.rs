use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about)]
struct Cli {
    #[arg(value_name = "CLIPPINGS_FILE")]
    file: PathBuf,

    #[arg(short, long, value_name = "TEMPLATE_FILE")]
    template: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    println!("Clippings file: {:?}", cli.file);

    if let Some(template) = cli.template {
        println!("Template file: {:?}", template);
    }
}
