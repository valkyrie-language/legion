
use super::*;

#[derive(Parser)]
#[command(about, long_about = None)]
pub struct AddCommand {}

impl AddCommand {
    pub async fn run(&self, p0: &LegionOptions) -> anyhow::Result<()> {
        todo!()
    }
}

