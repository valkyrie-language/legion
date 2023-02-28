use crate::{AddCommand, InstallCommand, NewCommand};
use clap::Subcommand;

pub mod cmd_add;
pub mod cmd_install;
pub mod cmd_new;

#[derive(Subcommand)]
pub enum LegionCommands {
    #[command(alias = "init")]
    New(NewCommand),
    /// add to local
    #[command(short_flag = 'a')]
    Add(AddCommand),
    
    Clone(NewCommand),

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
    pub fn run(&self) {}
}
