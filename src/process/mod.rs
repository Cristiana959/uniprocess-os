use std::{env, path::Path, thread};

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::{Engine, Linker, Module, Store};

pub(crate) fn create_process(module_path: String) {
    let engine = Engine::default();
    let module = Module::from_file(&engine, module_path).unwrap();

    let mut linker = Linker::new(&engine);
    wasi_common::sync::add_to_linker(&mut linker, |t| t).unwrap();
    // let pre = linker.instantiate_pre(&module).unwrap();

    let dir = Dir::open_ambient_dir(
        "/Users/cristianaandrei/facultate/uniprocess-os/src",
        ambient_authority(),
    )
    .unwrap();
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .preopened_dir(dir.try_clone().unwrap(), ".")
        .unwrap()
        .inherit_args()
        .unwrap()
        .build();

    let mut store = Store::new(&engine, wasi);

    let linking1 = linker.instantiate(&mut store, &module).unwrap();

    let run = linking1
        .get_typed_func::<(), ()>(&mut store, "_start")
        .unwrap();

    let working_directory = Path::new("/Users/cristianaandrei/facultate/uniprocess-os/src"); // The path you want to set as the cwd
    env::set_current_dir(working_directory).unwrap();
    println!("Current dir is {:?}", env::current_dir());
    let _ = thread::spawn(move || {
        run.call(&mut store, ()).unwrap();
    })
    .join();
}
