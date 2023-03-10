use clap::Args;

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
