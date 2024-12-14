// rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;

use clap::Parser;
use rcli_fea::cli::{Opts, SubCommand};
use rcli_fea::process::process_csv;
use rcli_fea::process::process_genpass;

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
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
    }

    // 所有操作成功完成
    Ok(())
}
