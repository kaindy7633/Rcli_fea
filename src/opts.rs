use clap::Parser;
use std::path::Path;

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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// 验证输入文件是否存在
///
/// # 参数
/// * `file_name` - 要验证的文件路径字符串
///
/// # 返回值
/// * `Ok(String)` - 如果文件存在，返回文件名字符串
/// * `Err(String)` - 如果文件不存在，返回错误信息
fn verify_input_file(file_name: &str) -> Result<String, &'static str> {
    // 使用标准库的 Path 检查文件是否存在
    if Path::new(file_name).exists() {
        // 文件存在，返回文件名
        Ok(file_name.into())
    } else {
        // 文件不存在，返回错误信息
        Err("File does not exist!")
    }
}
