pub mod base64;
pub mod csv;
pub mod genpass;

use std::path::Path;

pub use self::base64::{Base64Format, Base64SubCommand};
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
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

/// 验证输入文件是否存在
///
/// # 参数
/// * `file_name` - 要验证的文件路径字符串
///
/// # 返回值
/// * `Ok(String)` - 如果文件存在，返回文件名字符串
/// * `Err(String)` - 如果文件不存在，返回错误信息
pub fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exist"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("nonexistent.txt"),
            Err("File does not exist")
        );
    }
}
