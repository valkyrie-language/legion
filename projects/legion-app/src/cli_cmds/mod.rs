use crate::{
    AddCommand, InstallCommand, LegionOptions, NewCommand,
    cli_cmds::{cmd_decode::DecodeCommand, cmd_run::ExecuteCommand},
    helpers::ensure_parent,
};
use clap::{Parser, Subcommand};
use std::{
    collections::HashMap,
    fs::File,
    io::{Sink, Write},
    path::{Path, PathBuf},
};
pub mod cmd_add;
pub mod cmd_build;
pub mod cmd_decode;
pub mod cmd_encode;
pub mod cmd_install;
pub mod cmd_new;
pub mod cmd_run;

#[derive(Subcommand)]
pub enum LegionCommands {
    #[command(aliases = ["initialization", "init"])]
    New(NewCommand),
    /// add to local
    #[command(short_flag = 'a')]
    Add(AddCommand),
    /// Execute script or command
    #[command(short_flag = 'e', aliases = ["exe", "exec"])]
    Execute(ExecuteCommand),
    Clone(NewCommand),
    /// Decode wasm to wat
    Decode(DecodeCommand),
    /// add to global
    #[command(short_flag = 'i')]
    Install(InstallCommand),
    /// Recursively update indirect dependencies
    #[command(short_flag = 'u')]
    Update(InstallCommand),
    /// Update direct dependencies
    Upgrade(InstallCommand),
    #[command(short_flag = 'p')]
    Publish(InstallCommand),
}

impl LegionCommands {
    pub async fn run(&self, args: &LegionOptions) -> anyhow::Result<()> {
        match self {
            Self::New(cmd) => cmd.run(args).await?,
            Self::Add(cmd) => cmd.run(args).await?,
            Self::Clone(cmd) => cmd.run(args).await?,
            Self::Install(cmd) => cmd.run(args).await?,
            Self::Update(cmd) => cmd.run(args).await?,
            Self::Upgrade(cmd) => cmd.run(args).await?,
            Self::Publish(cmd) => cmd.run(args).await?,
            Self::Decode(cmd) => cmd.run(args).await?,
            Self::Execute(cmd) => cmd.run(args).await?,
        }
        Ok(())
    }
}
