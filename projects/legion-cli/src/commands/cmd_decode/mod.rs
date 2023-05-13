use crate::{LegionError, commands::LegionArguments};
use clap::Parser;
use wasmprinter::PrintFmtWrite;
use wat::GenerateDwarf;

#[derive(Debug, Parser)]
pub struct CommandDecode {
    #[arg(short, long, value_name = "FILE")]
    skeleton_only: bool,
    #[arg(short, long, value_name = "FILE")]
    indent_text: String,
    #[arg(short, long, value_name = "FILE")]
    fold_instructions: bool,
}

impl CommandDecode {
    pub async fn run(self, args: &LegionArguments) -> Result<(), LegionError> {
        let input = [];
        let mut parser = wasmprinter::Config::new();
        parser.name_unnamed(true);
        parser.print_offsets(false);
        parser.print_skeleton(self.skeleton_only);
        parser.indent_text(self.indent_text);
        parser.fold_instructions(self.fold_instructions);
        let mut dst = String::new();
        parser.print(&input, &mut PrintFmtWrite(&mut dst))?;

        Ok(())
    }
}
