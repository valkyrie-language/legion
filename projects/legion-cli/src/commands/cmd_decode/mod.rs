use crate::{LegionError, commands::LegionArguments};
use clap::Parser;
use wasmprinter::PrintFmtWrite;
use wat::GenerateDwarf;

#[derive(Debug, Parser)]
pub struct CommandDecode {
    input: String,
    #[arg(long)]
    skeleton_only: bool,
    #[arg(long, default_value = "4", value_name = "INDENT_LEVEL")]
    indent: usize,
    #[arg(long="indent-text")]
    _indent_text: Option<String>,
    #[arg(short, long)]
    fold_instructions: bool,
    #[arg(long)]
    print: bool,
    #[arg(long)]
    dry_run: bool,
}

impl CommandDecode {
    pub async fn run(&self, _: &LegionArguments) -> Result<(), LegionError> {
        let bytes = std::fs::read(&self.input)?;
        let mut parser = wasmprinter::Config::new();
        parser.name_unnamed(true);
        parser.print_offsets(false);
        parser.print_skeleton(self.skeleton_only);
        parser.indent_text(self.indent_text());
        parser.fold_instructions(self.fold_instructions);
        let mut dst = String::new();
        parser.print(&bytes, &mut PrintFmtWrite(&mut dst))?;
        if self.print {
            println!("{}", dst);
        }
        Ok(())
    }
    pub fn indent_text(&self) -> String {
        match self._indent_text.as_ref() {
            Some(s) => s.to_string(),
            None => " ".repeat(self.indent),
        }
    }
}
