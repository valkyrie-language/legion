use crate::bindings::{self, DecodeConfig, EncodeConfig, Guest, ToolsError, export};
use std::io::Sink;
use wasmprinter::PrintFmtWrite;
use wat::GenerateDwarf;

export!(ToolsContext with_types_in bindings);

pub struct ToolsContext {}

impl Guest for ToolsContext {
    fn encode_wasm(s: String, config: EncodeConfig) -> Result<Vec<u8>, ToolsError> {
        let mut parser = wat::Parser::new();
        if config.generate_dwarf {
            parser.generate_dwarf(GenerateDwarf::Full);
        }
        Ok(parser.parse_str(None, s)?)
    }

    fn decode_wasm(s: Vec<u8>, config: DecodeConfig) -> Result<String, ToolsError> {
        let mut parser = wasmprinter::Config::new();
        parser.print_offsets(false);
        parser.print_skeleton(config.skeleton_only);
        parser.name_unnamed(config.name_unnamed);
        parser.fold_instructions(config.fold_instructions);
        let mut dst = String::new();
        parser.print(&s, &mut PrintFmtWrite(&mut dst))?;
        Ok(dst)
    }
}
