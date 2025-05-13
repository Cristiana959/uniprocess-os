use std::thread;

use wasi_common::sync::WasiCtxBuilder;
use wasmtime::{Engine, Linker, Module, Store};

pub(crate) fn create_process(module_path: String) {
    let engine = Engine::default();
    let module = Module::from_file(&engine, module_path).unwrap();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |t| t).unwrap();
    // let pre = linker.instantiate_pre(&module).unwrap();

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .unwrap()
        .build();

    let mut store = Store::new(&engine, wasi);

    let linking1 = linker.instantiate(&mut store, &module).unwrap();
    let run = linking1
        .get_typed_func::<(), ()>(&mut store, "_start")
        .unwrap();
    let _ = thread::spawn(move || {
        run.call(&mut store, ()).unwrap();
    })
    .join();
}
