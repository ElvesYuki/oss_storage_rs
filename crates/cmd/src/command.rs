mod start;

use server;
use anyhow::Result;
use clap::Subcommand;

// 客户端命令
#[derive(Subcommand)]
#[command(infer_subcommands = true)]
pub enum CliCommand {

    /// 启动命令
    Start(start::StartCommand),

    /// 默认命令，启动服务
    Default,

}

impl CliCommand {
    pub fn run(self) -> Result<()> {

        match self {
            CliCommand::Start(cmd) => {
                cmd.run()
            },
            CliCommand::Default => {
                server::start()
            }
        }


    }
}

