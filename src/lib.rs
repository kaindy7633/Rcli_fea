mod opts;
mod process;

pub use opts::{CsvOpts, Opts, SubCommand};
pub use process::process_csv;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use opts::OutputFormat;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_process_csv() -> Result<()> {
        // 创建临时目录用于测试
        let temp_dir = tempdir()?;

        // 创建测试用的 CSV 文件
        let input_path = temp_dir.path().join("test.csv");
        fs::write(
            &input_path,
            r#"Name,Age,Position,DOB,Nationality,Kit Number
Alice,20,Engineer,"Apr 18, 1990 (29)",Poland,1
Bob,25,Designer,"Nov 10, 1992 (26)",Italy,37"#,
        )?;

        // 设置输出文件路径
        let output_path = temp_dir.path().join("output.json");

        // 使用 OutputFormat 枚举而不是字符串
        let format = OutputFormat::Json;

        // 测试处理函数
        process_csv(
            input_path.to_str().unwrap(),
            output_path.to_str().unwrap().to_string(),
            format,
        )?;

        // 验证输出文件存在
        assert!(output_path.exists());

        // 验证输出内容（根据你的实际需求调整）
        let content = fs::read_to_string(&output_path)?;
        assert!(content.contains("Alice"));

        Ok(())
    }
}
