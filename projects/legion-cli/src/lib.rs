pub mod commands;
mod errors;

// use inquire::{Text, validator::{StringValidator, Validation}, CustomUserError};
pub use crate::errors::LegionError;
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

impl LegionCLI {
    pub async fn run(self) -> Result<(), LegionError> {
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

use crate::commands::{LegionArguments, LegionCommands};
use dialoguer::{Select, theme::ColorfulTheme};

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
