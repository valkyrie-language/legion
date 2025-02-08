use crate::{LegionError, commands::LegionArguments};
use clap::Parser;
use dialoguer::{Select, theme::ColorfulTheme};
use std::path::Path;

// legion update --dependencies -d
// legion update --interactive -i
// legion update --major       -M
// legion update --minor       -m
// legion update --patch       -P
// legion update --pre-release -p
#[derive(Debug, Parser)]
pub struct CommandUpdate {
    #[arg(short, long, value_name = "BINARY")]
    interactive: Option<String>,
}

impl CommandUpdate {
    pub async fn run(&self, args: &LegionArguments) -> Result<(), LegionError> {
        // single file mode
        match self.interactive.as_ref() {
            Some(s) if Path::new(s).exists() => {
                return self.run_single_binary(s, args);
            }
            _ => {}
        }
        // package mode
        let exe = self.select_executable()?;
        println!("Running {}!", exe);
        Ok(())
    }

    pub fn select_executable(&self) -> Result<String, LegionError> {
        if let Some(s) = self.interactive.as_ref() {
            return Ok(s.to_owned());
        }
        let available = vec!["@package1/bin1", "@package1/bin2", "@package2/bin3", "@package2/bin4"];
        if let [one] = available.as_slice() {
            return Ok(one.to_string());
        }
        println!("There are multiple executable files in the directory.");
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the executable:")
            .default(0)
            .items(&available)
            .interact()?;
        Ok(available[selection].to_owned())
    }

    fn run_single_binary(&self, path: &str, _: &LegionArguments) -> Result<(), LegionError> {
        println!("Running {}!", path);
        Ok(())
    }
}
