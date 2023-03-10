use clap::Parser;
use crate::LegionOptions;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct InstallCommand {
    /// Install global
    #[arg(short, long, action = clap::ArgAction::Count)]
    timing: u8,
}

impl InstallCommand {
    pub async fn run(&self, p0: &LegionOptions) -> anyhow::Result<()> {
        todo!()
    }
}
