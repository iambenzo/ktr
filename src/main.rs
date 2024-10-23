use clap::Parser;
use kindle_clippings::run;
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
    run(cli.file, cli.template);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
