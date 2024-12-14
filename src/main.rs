// rcli csv -i input.csv -o output.json --header -d ','

use anyhow::Result;

use clap::Parser;
use rcli_fea::cli::{Base64SubCommand, Opts, SubCommand};
use rcli_fea::process::b64::{process_decode, process_encode};
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
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }

    // 所有操作成功完成
    Ok(())
}
