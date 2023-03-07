use super::*;

#[tokio::test]
async fn decode() -> anyhow::Result<()> {
    let mut here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = here.join("src/cli_cmds/cmd_decode/decode_component.wasm");
    let cmd = DecodeCommand {
        input: input.to_string_lossy().to_string(),
        output: None,
        skeleton: false,
        name_unnamed: false,
        fold_instructions: false,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    let cmd = DecodeCommand {
        input: input.to_string_lossy().to_string(),
        output: Some(input.with_file_name("decode_component_skeleton.wat").to_string_lossy().to_string()),
        skeleton: true,
        name_unnamed: false,
        fold_instructions: false,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    let cmd = DecodeCommand {
        input: input.to_string_lossy().to_string(),
        output: Some(input.with_file_name("decode_component_fold.wat").to_string_lossy().to_string()),
        skeleton: false,
        name_unnamed: false,
        fold_instructions: true,
        print: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await?;

    Ok(())
}
