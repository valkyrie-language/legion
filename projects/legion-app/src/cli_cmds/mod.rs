use crate::{AddCommand, InstallCommand, LegionOptions, NewCommand, helpers::ensure_parent};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};
use url::Url;
use wat::GenerateDwarf;

pub mod cmd_add;
pub mod cmd_decode;
pub mod cmd_encode;
pub mod cmd_install;
pub mod cmd_new;
#[derive(Subcommand)]
pub enum LegionCommands {
    #[command(aliases = ["initialization", "init"])]
    New(NewCommand),
    /// add to local
    #[command(short_flag = 'a')]
    Add(AddCommand),
    Clone(NewCommand),
    /// Encode wat to wasm
    WasmEncode(NewCommand),
    /// Decode wasm to wat
    WasmDecode(NewCommand),
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
    pub fn run(&self, args: &LegionOptions) -> anyhow::Result<()> {
        match self {
            Self::New(_) => {}
            Self::Add(_) => {}
            Self::Clone(_) => {}
            Self::Install(_) => {}
            Self::Update(_) => {}
            Self::Upgrade(_) => {}
            Self::Publish(_) => {}
            Self::WasmEncode(_) => {}
            Self::WasmDecode(_) => {}
        }
        Ok(())
    }
}
