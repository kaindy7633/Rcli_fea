use clap::Parser;

#[derive(Debug, Clone, Copy, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 8)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
