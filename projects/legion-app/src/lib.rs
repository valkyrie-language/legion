mod cli_cmds;
mod helpers;

use clap::{Args, Parser, Subcommand};
pub use cli_cmds::{
    LegionCommands, cmd_add::AddCommand, cmd_decode::DecodeCommand,  cmd_install::InstallCommand,
    cmd_new::NewCommand,
};
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct LegionCLI {
    #[command(flatten)]
    options: LegionOptions,
    #[command(subcommand)]
    command: Option<LegionCommands>,
}
/// Legion global options
#[derive(Debug, Args)]
pub struct LegionOptions {
    /// Timing Tracing Debugging
    ///
    /// `-t`: show time
    /// `-tt`: show time and memory
    /// `-ttt`: show time, memory, and stack
    #[arg(short, long, action = clap::ArgAction::Count)]
    timing: u8,
    /// Skip confirmation before irreversible side effects
    ///
    /// e.g. `rm`, `publish`
    #[arg(long)]
    yes: bool,
}

impl LegionCLI {
    pub async fn run(&self) -> anyhow::Result<()> {
        if self.options.timing != 0 {
            println!("Timing mode is on");
        }
        else {
            println!("Timing mode is off");
        }
        let Self { options, command } = self;
        match command {
            Some(c) => c.run(options).await?,
            None => {}
        }
        if self.options.yes {
            println!("Dry run mode is on");
        }
        Ok(())
    }
}
