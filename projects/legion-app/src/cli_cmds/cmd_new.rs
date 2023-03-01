use clap::Parser;

/// Create new project or subproject with template
#[derive(Parser)]
#[command(about, long_about = None)]
pub struct NewCommand {
    /// Package name
    pub name: Option<String>,
    #[arg(short, long)]
    pub template: Option<String>,
}
