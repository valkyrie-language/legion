use std::path::Path;
use tokio::fs::create_dir_all;

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
