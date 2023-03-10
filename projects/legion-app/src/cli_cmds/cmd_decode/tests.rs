use crate::helpers::project_path;
use super::*;

#[tokio::test]
async fn decode() -> anyhow::Result<()> {
    let cmd = DecodeCommand {
        input: project_path("src/cli_cmds/cmd_decode/decode_component.wasm"),
        output: None,
        skeleton: false,
        name_unnamed: false,
        fold_instructions: false,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    let cmd = DecodeCommand {
        input: project_path("src/cli_cmds/cmd_decode/decode_component.wasm"),
        output: Some(project_path("src/cli_cmds/cmd_decode/decode_component_skeleton.wat")),
        skeleton: true,
        name_unnamed: false,
        fold_instructions: false,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    let cmd = DecodeCommand {
        input: project_path("src/cli_cmds/cmd_decode/decode_component.wasm"),
        output: Some(project_path("src/cli_cmds/cmd_decode/decode_component_fold.wat")),
        skeleton: false,
        name_unnamed: false,
        fold_instructions: true,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    Ok(())
}
