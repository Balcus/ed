use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "Balcus Bogdan",
    version,
    about = "Simple terminal text editor",
    long_about = None,
    after_help = "Giuhub page: https://github.com/Balcus/ed"
)]
pub struct Args {
    /// Name of file to open in the editor
    #[arg(short = 'f', long = "file", value_name = "FILE")]
    pub file_name: Option<String>,
}

pub(crate) fn parse_args() -> Option<String> {
    let args = Args::parse();
    args.file_name
}