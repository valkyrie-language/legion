mod cli_cmds;

use clap::{Args, Parser, Subcommand};
pub use cli_cmds::{LegionCommands, cmd_add::AddCommand, cmd_install::InstallCommand, cmd_new::NewCommand};
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct LegionCLI {
    #[command(flatten)]
    options: LegionOptions,
    #[command(subcommand)]
    command: Option<LegionCommands>,
}
/// Legion global options
#[derive(Args)]
struct LegionOptions {
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
    pub fn run(&self) {
        if self.options.timing != 0 {
            println!("Timing mode is on");
        }
        else {
            println!("Timing mode is off");
        }
        match &self.command {
            Some(c) => c.run(),
            None => {}
        }
        if self.options.yes {
            println!("Dry run mode is on");
        }
    }
}

fn main() {
    LegionCLI::parse().run()
}
