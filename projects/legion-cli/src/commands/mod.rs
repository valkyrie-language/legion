use crate::LegionError;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
mod run_decode;
mod run_encode;
mod run_polyfill;

pub use self::run_encode::RunEncode;

#[derive(Debug, Subcommand)]
pub enum LegionCommands {
    /// does testing things
    Encode(RunEncode),
    /// does testing things
    Decode(RunEncode),
    #[command(alias = "shim")]
    Polyfill(RunEncode),
}

#[derive(Debug, Args)]
pub struct LegionArguments {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}


impl LegionCommands {
    pub async fn run(self, arguments: &LegionArguments) -> Result<(), LegionError> {
        match self {
            Self::Encode(cmd) => cmd.run(arguments).await,
            Self::Decode(cmd) => cmd.run(arguments).await,
            Self::Polyfill(cmd) => cmd.run(arguments).await,
        }
    }
}
