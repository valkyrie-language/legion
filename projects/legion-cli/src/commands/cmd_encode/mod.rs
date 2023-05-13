use crate::{LegionError, commands::LegionArguments};
use clap::{Parser, builder::Str};
use std::path::{Path, PathBuf};
use wat::GenerateDwarf;

/// encode `wat`, `wast` to wasm
#[derive(Debug, Parser)]
pub struct CommandEncode {
    /// Input `wat` file path
    #[arg(short, long, value_name = "FILE")]
    input: String,
    /// Output `wasm` file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,
    /// Skip output if file already exists
    #[arg(long)]
    skip_override: bool,
    /// Generate DWARF debugging information
    #[arg(short = 'd', long)]
    generate_dwarf: bool,
    #[arg(long)]
    dry_run: bool,
}

impl CommandEncode {
    pub async fn run(&self, args: &LegionArguments) -> Result<(), LegionError> {
        let input = std::fs::read_to_string(&self.input)?;
        let mut parser = wat::Parser::new();
        if self.generate_dwarf {
            parser.generate_dwarf(GenerateDwarf::Full);
        }
        let bytes = parser.parse_str(None, input)?;
        let output = self.guess_output();
        if self.skip_override {
            if output.exists() {
                println!("{}", "Skipping override");
                return Ok(());
            }
            else if self.dry_run {
                println!("{}", "Finish dry run");
                return Ok(());
            }
            else {
                std::fs::write(&output, bytes)?;
            }
        }
        Ok(())
    }
    pub fn guess_output(&self) -> PathBuf {
        let input = Path::new(&self.input);
        match self.output.as_ref() {
            Some(s) => PathBuf::from(s),
            None => input.with_extension("wasm"),
        }
    }
}
