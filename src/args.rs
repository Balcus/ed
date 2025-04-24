use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "Balcus Bogdan",
    version,
    about = "KEYBOARD SHORTCUTS:
* Ctrl + q - Quit the editor
* Ctrl + s - Save file
* Ctrl + x - Delete current line
* Ctrl + ← - Jump to previous word
* Ctrl + → - Jump to next word
* Home - Go to start of line
* End - Go to end of line
* PageUp/PageDown - Scroll up/down",
    long_about = None,
    after_help = "Giuhub repo: https://github.com/Balcus/ed"
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