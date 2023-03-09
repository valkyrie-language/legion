use crate::{LegionOptions, helpers::ensure_parent};
use clap::Parser;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};
use wat::GenerateDwarf;

mod run_wasm;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct RunCommand {
    /// input wat file
    input: Option<String>,
    /// Run which package in the workspace
    #[arg(short, long)]
    package: String,
}

impl RunCommand {
    pub async fn run(&self, args: &LegionOptions) -> anyhow::Result<()> {
        match self.input.as_ref() {
            // single file wasm
            Some(s) if s.ends_with(".wasm") => {
                let mut parser = wat::Parser::new();
                parser.generate_dwarf(GenerateDwarf::Full);
                let wasm_bytes = parser.parse_file(s)?;
                self.run_wasm(&wasm_bytes).await?;
            },
            // simple file wat
            Some(s) if s.ends_with(".wat") => {
                let mut parser = wat::Parser::new();
                parser.generate_dwarf(GenerateDwarf::Full);
                let wasm_bytes = parser.parse_file(s)?;
                self.run_wasm(&wasm_bytes).await?;
            },
            // simple file valkyrie
            Some(s) if s.ends_with(".valkyrie") => {
            },
            // simple file valkyrie
            Some(s) if s.ends_with(".vk") => {
            },
            // run dir project
            Some(s) if s.ends_with("/") => {
            },
            // other case
            Some(s) => {

            }
            None => {
                println!("run with current workspace")
            }
        }
        Ok(())
    }
}
