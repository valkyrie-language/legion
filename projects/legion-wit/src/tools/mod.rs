use crate::bindings::{self, export, DecodeConfig, EncodeConfig, Guest};

export!(ToolsContext with_types_in bindings);

pub struct ToolsContext {}

impl Guest for ToolsContext {
    fn encode_wasm(s: String, config: EncodeConfig) -> Result<Vec<u8>, ()> {
        Ok(vec![])
    }

    fn decode_wasm(s: Vec<u8>, config: DecodeConfig) -> Result<String, ()> {
        Ok(String::new())
    }
}
