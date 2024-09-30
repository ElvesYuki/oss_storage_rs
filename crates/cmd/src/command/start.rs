
use clap::Parser;
use anyhow::{Result, };

/// 转换参数
#[derive(Debug, Parser)]
pub struct StartCommand {

    /// 待转换的文件地址，输入文件格式为xlsx
    #[arg(short, long)]
    input_path: Option<String>,

    /// 转换后的输出文件地址，输出文件格式为json
    #[arg(short, long)]
    output_path: Option<String>,
}

impl StartCommand {
    pub fn run(self) -> Result<()> {
        server::start()
    }
}