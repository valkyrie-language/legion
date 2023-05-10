use crate::{LegionError, commands::LegionArguments};
use clap::Parser;
use wat::GenerateDwarf;

/// encode `wat`, `wast` to wasm
#[derive(Debug, Parser)]
pub struct RunEncode {
    /// Input file path
    input: String,
    #[arg(short, long, value_name = "FILE")]
    generate_dwarf: bool,
}

impl RunEncode {
    pub async fn run(self, args: &LegionArguments) -> Result<(), LegionError> {
        let input = std::fs::read_to_string(self.input)?;
        let mut parser = wat::Parser::new();
        if self.generate_dwarf {
            parser.generate_dwarf(GenerateDwarf::Full);
        }
        let bytes = parser.parse_str(None, input)?;
        Ok(())
    }
}
