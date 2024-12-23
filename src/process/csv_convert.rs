use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

use crate::cli::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 初始化 CSV 读取器，从指定文件读取数据
    let mut reader = Reader::from_path(input)?;

    // 预分配容量为 128 的动态数组，用于存储解析后的数据
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    // 迭代处理 CSV 文件的每一行数据
    // reader.deserialize() 自动将 CSV 记录转换为 Player 结构体
    for result in reader.records() {
        // 如果解析失败，使用 ? 运算符向上传播错误
        // headers.iter() -> 使用 headers 迭代器
        // record.iter() -> 使用 record 迭代器
        // zip() -> 将两个迭代器合并为一个元组迭代器 [(header, record), ...]
        // collect::<Value>() -> 将元组迭代器转换为 JSON 值
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        println!("{:?}", record);
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // TODO 目前转换为Toml会失败，需要解决
        OutputFormat::Toml => toml::to_string(&ret)?,
    };

    // 将 JSON 数据写入输出文件
    fs::write(output, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

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
