use super::*;
use crate::helpers::arg_io::InputOutputArgs;
use js_component_bindgen::{BindingsMode, InstantiationMode, TranspileOpts, transpile};
use std::io::Sink;
use wasmprinter::{PrintFmtWrite, PrintIoWrite};

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct DecodeCommand {
    /// input wat file
    input: String,
    /// output wasm file name
    #[arg(short, long)]
    output: Option<String>,
    /// Whether or not to print only a "skeleton" which skips function bodies,
    /// data segment contents, element segment contents, etc.
    #[arg(long)]
    skeleton: bool,
    /// Assign names to all unnamed items.
    ///
    /// If enabled then any previously unnamed item will have a name synthesized
    /// that looks like `$#func10` for example. The leading `#` indicates that
    /// it's `wasmprinter`-generated. The `func` is the namespace of the name
    /// and provides extra context about the item when referenced. The 10 is the
    /// local index of the item.
    ///
    /// Note that if the resulting text output is converted back to binary the
    /// resulting `name` custom section will not be the same as before.
    #[arg(long)]
    name_unnamed: bool,
    /// Print instructions in folded form where possible.
    ///
    /// This will cause printing to favor the s-expression (parenthesized) form
    /// of WebAssembly instructions. For example this output would be generated
    /// for a simple `add` function:
    ///
    /// ```wasm
    /// (module
    ///     (func $foo (param i32 i32) (result i32)
    ///         (i32.add
    ///             (local.get 0)
    ///             (local.get 1))
    ///     )
    /// )
    /// ```
    #[arg(long)]
    fold_instructions: bool,
    /// with-print
    #[arg(long)]
    print: bool,
    /// dry run
    #[arg(long)]
    dry_run: bool,

    #[arg(long)]
    js: bool,
}

impl DecodeCommand {
    pub async fn run(&self, args: &LegionOptions) -> anyhow::Result<()> {
        if self.js { self.transpile(args).await } else { self.decode(args).await }
    }
    pub async fn decode(&self, args: &LegionOptions) -> anyhow::Result<()> {
        let input_path = Path::new(&self.input);
        let input = tokio::fs::read(input_path).await?;
        let mut parser = wasmprinter::Config::new();
        parser.print_offsets(false);
        parser.print_skeleton(self.skeleton);
        parser.name_unnamed(self.name_unnamed);
        parser.fold_instructions(self.fold_instructions);

        if self.print {
            let mut dst = String::new();
            parser.print(&input, &mut PrintFmtWrite(&mut dst))?;
            println!("{}", dst)
        }

        let output = self.make_output_name(input_path).await?;
        if self.dry_run {
            let mut file = Sink::default();
            parser.print(&input, &mut PrintIoWrite(&mut file))?
        }
        else {
            let mut file = std::io::BufWriter::new(std::fs::File::create(output)?);
            parser.print(&input, &mut PrintIoWrite(&mut file))?
        }
        Ok(())
    }
    pub async fn transpile(&self, args: &LegionOptions) -> anyhow::Result<()> {
        let input_path = Path::new(&self.input);
        let input = tokio::fs::read(input_path).await?;
        let cfg = TranspileOpts {
            name: "index".to_string(),
            no_typescript: false,
            instantiation: Some(InstantiationMode::Async),
            import_bindings: Some(BindingsMode::Js),
            map: None,
            no_nodejs_compat: false,
            base64_cutoff: 0,
            tla_compat: true,
            valid_lifting_optimization: false,
            tracing: false,
            no_namespaced_exports: true,
            multi_memory: true,
        };
        let result = transpile(&input, cfg)?;

        let output = self.make_output_dir(input_path).await?;
        for (path, bytes) in result.files {
            let path = output.join(path);
            ensure_parent(&path).await?;
            if self.dry_run {
            }
            else {
                let mut file = tokio::fs::File::create(path).await?;
                file.write_all(&bytes).await?;
            }
        }

        Ok(())
    }

    async fn make_output_name(&self, input: &Path) -> anyhow::Result<PathBuf> {
        match self.output.as_ref() {
            None => Ok(input.with_extension("wat")),
            Some(s) => {
                let path = PathBuf::from(s);
                ensure_parent(&path).await?;
                Ok(path)
            }
        }
    }
    async fn make_output_dir(&self, input: &Path) -> anyhow::Result<PathBuf> {
        match self.output.as_ref() {
            None => {
                let parent = parent_path(input)?;
                match input.file_stem().and_then(|s| s.to_str()) {
                    Some(s) => Ok(parent.join(s)),
                    None => Err(anyhow::anyhow!("Invalid file name")),
                }
            }
            Some(s) => {
                let path = PathBuf::from(s);
                if path.is_dir() {
                    ensure_parent(&path).await?;
                    Ok(path)
                }
                else {
                    Ok(parent_path(&path)?.to_path_buf())
                }
            }
        }
    }
}

pub fn parent_path(path: &Path) -> anyhow::Result<&Path> {
    match path.parent() {
        Some(s) => Ok(s),
        None => Err(anyhow::anyhow!("No parent directory for file")),
    }
}
