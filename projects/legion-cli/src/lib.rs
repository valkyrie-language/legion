mod errors;
mod tools;
// use inquire::{Text, validator::{StringValidator, Validation}, CustomUserError};
pub use crate::errors::ToolsError;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LegionCLI {
    #[command(subcommand)]
    commands: Option<LegionCommands>,
    #[command(flatten)]
    arguments: LegionArguments,
}

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
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Encode(RunEncode),
}

#[derive(Debug, Parser)]
pub struct RunEncode {
    #[arg(short, long, value_name = "FILE")]
    generate_dwarf: bool,
}

impl RunEncode {
    pub async fn run(self, args: &LegionArguments) -> Result<(), ToolsError> {
        let input = "";
        let mut parser = wat::Parser::new();
        if self.generate_dwarf {
            parser.generate_dwarf(GenerateDwarf::Full);
        }
        let bytes = parser.parse_str(None, input)?;
        Ok(())
    }
}
impl LegionCommands {
    pub async fn run(self, arguments: &LegionArguments) -> Result<(), ToolsError> {
        match self {
            LegionCommands::Test { list } => {
                println!("Testing!");
                Ok(())
            }
            LegionCommands::Encode(cmd) => cmd.run(arguments).await,
        }
    }
}

impl LegionCLI {
    pub async fn run(self) -> Result<(), ToolsError> {
        println!("{:?}", self.arguments);
        let Self { commands, arguments } = self;
        match commands {
            Some(s) => s.run(&arguments).await?,
            None => {
                main();
            }
        }
        Ok(())
    }
}

use dialoguer::{Select, theme::ColorfulTheme};
use wat::GenerateDwarf;

fn main() {
    let selections = &["Ice Cream", "Vanilla Cupcake", "Chocolate Muffin", "A Pile of sweet, sweet mustard"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        println!("Enjoy your {}!", selections[selection]);
    }
    else {
        println!("You didn't select anything!");
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor, hint it might be on the second page")
        .default(0)
        .max_length(2)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}
