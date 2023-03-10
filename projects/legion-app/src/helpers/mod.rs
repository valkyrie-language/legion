use std::path::Path;
use tokio::fs::create_dir_all;

pub mod arg_io;

pub async fn ensure_parent(path: &Path) -> anyhow::Result<()> {
    match path.parent() {
        Some(s) if !s.exists() => {
            create_dir_all(s).await?;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
pub fn project_path(relative: &str) -> String {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    here.join(relative).to_string_lossy().to_string()
}
pub fn parent_path(path: &Path) -> anyhow::Result<&Path> {
    match path.parent() {
        Some(s) => Ok(s),
        None => Err(anyhow::anyhow!("No parent directory for file")),
    }
}
