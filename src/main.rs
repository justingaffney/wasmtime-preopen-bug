use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::*;
use wasmtime_wasi::bindings::sync::Command;
use wasmtime_wasi::{DirPerms, FilePerms, WasiCtx, WasiCtxBuilder, WasiView};

pub struct ComponentRunStates {
    pub wasi_ctx: WasiCtx,
    pub resource_table: ResourceTable,
}

impl WasiView for ComponentRunStates {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resource_table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

fn main() -> Result<()> {
    let config = Config::new();

    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()

        .preopened_dir("test_dirs/a", "/a", DirPerms::READ, FilePerms::READ)?
        .preopened_dir("test_dirs/b", "/b", DirPerms::READ, FilePerms::READ)?
        .preopened_dir("test_dirs/c", "/c", DirPerms::READ, FilePerms::READ)?
        .preopened_dir("test_dirs/d", "/d", DirPerms::READ, FilePerms::READ)?
        .preopened_dir("test_dirs/e", "/e", DirPerms::READ, FilePerms::READ)?

        .build();

    let state = ComponentRunStates {
        wasi_ctx,
        resource_table: ResourceTable::new(),
    };
    let mut store = Store::new(&engine, state);

    let component = Component::from_file(&engine, "component/target/wasm32-wasip1/release/preopened-dir-test.wasm")?;

    let command = Command::instantiate(&mut store, &component, &linker)?;

    let _ = command.wasi_cli_run().call_run(&mut store)?;

    Ok(())
}