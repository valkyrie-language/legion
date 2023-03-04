use clap::Parser;
use legion::LegionCLI;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LegionCLI::parse().run().await
}
