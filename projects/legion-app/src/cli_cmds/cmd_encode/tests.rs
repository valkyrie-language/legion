use super::*;


#[tokio::test]
async fn encode() {
    let mut here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cmd = EncodeCommand {
        input: here.join("src/cli_cmds/cmd_encode/encode_component.wat").to_string_lossy().to_string(),
        output: None,
        generate_dwarf: false,
        dry_run: false,
    };
    cmd.run(&LegionOptions { timing: 0, yes: false }).await.unwrap()
}