// rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;
use clap::Parser;
use rcli_fea::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    // 解析命令行参数
    let opts = Opts::parse();

    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
    }

    // 所有操作成功完成
    Ok(())
}
