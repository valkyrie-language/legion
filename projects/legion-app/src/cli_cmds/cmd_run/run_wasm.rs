use super::*;

use wasmtime::{
    Config, Engine, Store,
    component::{Component,  Linker, ResourceTable},
};
use wasmtime_wasi::{  WasiCtx, WasiView, add_to_linker_async};


pub struct LegionView {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl Default for LegionView {
    fn default() -> Self {
        Self {
            table: Default::default(),
            wasi: WasiCtx::builder().stdout(wasmtime_wasi::Stdout).stderr(wasmtime_wasi::Stderr).build(),
        }
    }
}

impl WasiView for LegionView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

impl ExecuteCommand {
    pub async fn run_wasm(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let mut config = Config::new();
        config
            .async_support(true)
            .wasm_function_references(true)
            .wasm_gc(true)
            .wasm_component_model(true)
            .wasm_component_model_multiple_returns(true);
        let engine = Engine::new(&config)?;
        let module = Component::new(&engine, bytes)?;
        let mut store = Store::new(&engine, LegionView::default());
        let mut linker = Linker::new(&engine);
        add_to_linker_async(&mut linker)?;
        let instance = linker.instantiate_async(&mut store, &module).await?;
        Ok(())
    }
}
