pub mod csv;
pub mod genpass;

pub use self::csv::{CsvOpts, OutputFormat};
pub use self::genpass::GenPassOpts;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    // #[command(name = "base64", about = "Base64 encode or decode")]
    // Base64(Base64SubCommand),
}
