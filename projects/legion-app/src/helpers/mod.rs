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
