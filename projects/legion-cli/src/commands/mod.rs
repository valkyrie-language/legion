use crate::ToolsError;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
mod run_decode;
mod run_encode;

pub use self::run_encode::RunEncode;

#[derive(Debug, Args)]
pub struct LegionArguments {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Debug, Subcommand)]
pub enum LegionCommands {
    /// does testing things
    Encode(RunEncode),
    /// does testing things
    Decode(RunEncode),
}

impl LegionCommands {
    pub async fn run(self, arguments: &LegionArguments) -> Result<(), ToolsError> {
        match self {
            Self::Encode(cmd) => cmd.run(arguments).await,
            Self::Decode(cmd) => cmd.run(arguments).await,
        }
    }
}
