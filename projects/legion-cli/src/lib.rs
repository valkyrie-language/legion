mod errors;
mod tools;
use inquire::{Text, validator::{StringValidator, Validation}, CustomUserError};
pub use crate::{errors::ToolsError};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct LegionCLI {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

impl LegionCLI {
    pub async fn run(&self) -> Result<(), ToolsError> {
        println!("{:?}", self);
        fn validator(input: &str) -> Result<Validation, CustomUserError> {
            if input.chars().count() > 140 {
                Ok(Validation::Invalid("You're only allowed 140 characters.".into()))
            } else {
                Ok(Validation::Valid)
            }
        }

        let status = Text::new("What are you thinking about?")
            .with_validator(validator)
            .prompt();

        match status {
            Ok(status) => println!("Your status is being published..."),
            Err(err) => println!("Error while publishing your status: {}", err),
        }
        Ok(())
    }
}


