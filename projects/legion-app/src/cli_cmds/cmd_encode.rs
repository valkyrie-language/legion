use super::*;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct EncodeCommand {
    /// input wat file
    input: String,
    /// output wasm file name
    #[arg(short, long)]
    output: Option<String>,
    /// dry run
    #[arg(long)]
    dry_run: bool,
}

impl EncodeCommand {
    pub async fn run(&self, args: &LegionOptions) -> anyhow::Result<()> {
        let input = Path::new(&self.input);
        let mut parser = wat::Parser::new();
        parser.generate_dwarf(GenerateDwarf::Full);
        let wasm_bytes = parser.parse_file(input)?;
        let output = self.make_output_name(input).await?;
        if !self.dry_run {
            let mut wasm = File::create(output).await?;
            wasm.write_all(&wasm_bytes).await?;
        }
        Ok(())
    }

    async fn make_output_name(&self, input: &Path) -> anyhow::Result<PathBuf> {
        match self.output.as_ref() {
            None => Ok(input.with_extension("wasm")),
            Some(s) => {
                let path = PathBuf::from(s);
                ensure_parent(&path).await?;
                Ok(path)
            }
        }
    }
}
