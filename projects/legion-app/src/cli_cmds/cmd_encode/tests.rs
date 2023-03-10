use crate::helpers::project_path;
use super::*;


#[tokio::test]
async fn encode() {
    let cmd = EncodeCommand {
        input: project_path("src/cli_cmds/cmd_encode/encode_component.wat"),
        output: None,
        generate_dwarf: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await.unwrap()
}


