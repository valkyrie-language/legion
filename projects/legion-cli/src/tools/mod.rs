// use crate::bindings::{self, DecodeConfig, EncodeConfig, Guest, PolyfillConfig, ToolsError, export};
// use js_component_bindgen::{BindingsMode, InstantiationMode, TranspileOpts};
// use std::collections::HashMap;
// use clap::{Error, Parser};
// use wasmprinter::PrintFmtWrite;
// use wat::GenerateDwarf;
// use crate::LegionCLI;
//
// export!(ToolsContext with_types_in bindings);
//
// pub struct ToolsContext {}
//
// impl Guest for ToolsContext {
//     fn run() -> Result<(), String> {
//         Ok(())
//     }
//

//
//     fn wasm_decode(input: Vec<u8>, config: DecodeConfig) -> Result<String, ToolsError> {

//     }
//
//     fn wasi_polyfill(input: Vec<u8>, config: PolyfillConfig) -> Result<Vec<(String, Vec<u8>)>, ToolsError> {
//         let mut map = HashMap::default();
//         map.insert("wasi:*".to_string(), "@bytecodealliance/preview2-shim/*".to_string());
//         map.insert("valkyrie:std-legacy/*".to_string(), "@valkyrie-language/std-legacy/*".to_string());
//         for (k, v) in config.shim {
//             map.insert(k, v);
//         }
//         let cfg = TranspileOpts {
//             name: config.name,
//             no_typescript: false,
//             instantiation: if config.instantiation { Some(InstantiationMode::Async) } else { None },
//             import_bindings: Some(BindingsMode::Js),
//             map: Some(map),
//             no_nodejs_compat: false,
//             base64_cutoff: 0,
//             tla_compat: false,
//             valid_lifting_optimization: !config.debug,
//             tracing: config.debug,
//             no_namespaced_exports: true,
//             multi_memory: true,
//             guest: config.guest,
//         };
//         let result = js_component_bindgen::transpile(&input, cfg)?;
//         Ok(result.files)
//     }
// }
