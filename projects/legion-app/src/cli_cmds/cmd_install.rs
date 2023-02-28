use clap::Parser;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct InstallCommand {
    /// Install global
    #[arg(short, long, action = clap::ArgAction::Count)]
    timing: u8,
}
