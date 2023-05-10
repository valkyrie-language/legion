use crate::{ToolsError, commands::LegionArguments};
use clap::Parser;
use wat::GenerateDwarf;

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
