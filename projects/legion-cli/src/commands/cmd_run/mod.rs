use crate::{LegionError, commands::LegionArguments};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};

#[derive(Debug, Parser)]
pub struct CommandRun {
    #[arg(short, long, value_name = "BINARY")]
    binary: Option<String>,
}

impl CommandRun {
    pub async fn run(&self, args: &LegionArguments) -> Result<(), LegionError> {
        let exe = self.select_executable()?;

        println!("Running {}!", exe);

        Ok(())
    }

    pub fn select_executable(&self) -> Result<String, LegionError> {
        match self.binary.as_ref() {
            Some(s) => Ok(s.to_owned()),
            None => {
                let selections = ["bin1", "bin2", "bin3", "bin4"];

                match selections {
                    [one] => Ok(one.to_owned()),
                    many => {
                        println!("There are multiple executable files in the directory.");
                        let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select the executable:")
                            .default(0)
                            .items(&selections)
                            .interact()?;
                        Ok(selections[selection].to_owned())
                    }
                }
            }
        }
    }
}
