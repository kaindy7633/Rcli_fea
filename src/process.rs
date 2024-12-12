use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
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

    // 将解析后的数据转换为格式化的 JSON 字符串
    let json = serde_json::to_string_pretty(&ret)?;
    // 将 JSON 数据写入输出文件
    fs::write(output, json)?;

    Ok(())
}
