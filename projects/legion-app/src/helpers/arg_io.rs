use crate::helpers::{parent_path};
use clap::Args;
use std::path::{Path, PathBuf};

/// Legion global options
#[derive(Debug, Args)]
pub struct InputOutputArgs {
    /// Timing Tracing Debugging
    input: String,
    /// Skip confirmation before irreversible side effects
    ///
    /// e.g. `rm`, `publish`
    output: Option<String>,
}

impl InputOutputArgs {
    #[cfg(test)]
    pub fn test_output(test: &str, input: &str, output: &str) -> InputOutputArgs {
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test = here.join(test);

        Self {
            input: test.join(input).to_string_lossy().to_string(),
            output: if output.is_empty() { None } else { Some(test.join(output).to_string_lossy().to_string()) },
        }
    }
}

impl InputOutputArgs {
    pub fn get_input_path(&self) -> anyhow::Result<PathBuf> {
        match Path::new(&self.input).canonicalize() {
            Ok(o) => Ok(o),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
    pub async fn get_input_bytes(&self) -> anyhow::Result<Vec<u8>> {
        Ok(std::fs::read(&self.input)?)
    }
    pub async fn get_input_text(&self) -> anyhow::Result<String> {
        Ok(std::fs::read_to_string(&self.input)?)
    }
    pub fn output_or_extension(&self, ext: &str) -> anyhow::Result<PathBuf> {
        match self.output.as_ref() {
            Some(s) => Ok(PathBuf::from(s)),
            None => Ok(Path::new(&self.input).with_extension(ext)),
        }
    }
    pub fn output_or_dir(&self) -> anyhow::Result<PathBuf> {
        let input = Path::new(&self.input);
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
                if path.is_dir() { Ok(path) } else { Ok(parent_path(&path)?.to_path_buf()) }
            }
        }
    }
}
