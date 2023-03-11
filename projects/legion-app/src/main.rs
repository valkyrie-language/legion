use clap::{ Parser};
use legion::LegionCLI;


fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
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
        })



}
