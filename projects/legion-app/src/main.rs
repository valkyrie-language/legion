use clap::{ Parser};
use legion::LegionCLI;

#[tokio::main]
async fn main() {
    match LegionCLI::try_parse() {
        Ok(cli) => match cli.run().await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}")
            }
        },
        Err(e) => {
            eprintln!("{e}")
        }
    }
}
