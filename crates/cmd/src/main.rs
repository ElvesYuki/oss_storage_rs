mod command;

use clap::Parser;
use anyhow::Result;
use crate::command::CliCommand;

fn main() {
    let root_cmd =  RootCmd::parse();

    if root_cmd.root.is_none() {
        CliCommand::Default.run().expect("程序解析异常，请检查文件内容");
        return;
    }

    root_cmd.run().expect("程序解析异常，请检查文件内容");
}

/// 管理系统
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RootCmd {

    #[command(subcommand)]
    root: Option<CliCommand>,

}

impl RootCmd {
    fn run(self) -> Result<()> {
        self.root.unwrap().run()
    }
}