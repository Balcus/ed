use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    author = "Balcus Bogdan",
    version,
    about = "Simple terminal text editor",
    long_about = None,
    after_help = "Github page: https://github.com/Balcus/ed"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Used to open the given files inside separate editor windows
    Open {
        /// Name of files to be opened
        #[arg(value_name = "FILE")]
        file_names: Vec<String>,
    },
}

pub fn parse_args() -> Vec<String> {
    let args = Args::parse();
    match args.command {
        Command::Open { file_names } => file_names
            .into_iter()
            .map(|f| f.trim().to_string())
            .collect(),
    }
}
