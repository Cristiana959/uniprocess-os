use chrono::{DateTime, Local};
use std::{
    collections::HashMap,
    env,
    path::Path,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

use cap_std::ambient_authority;
use cap_std::fs::Dir;
use serde::Serialize;
use wasi_common::sync::WasiCtxBuilder;
use wasmtime::{AsContext, Engine, Linker, Module, Store};

pub mod process_manager;

#[derive(Debug, Clone, Serialize)]
pub struct PCB {
    pub pid: u32,
    pub name: String,
    pub started_at: DateTime<Local>,
    pub status: String,
}

pub type ProcessTable = Arc<Mutex<HashMap<u32, PCB>>>;

pub(crate) fn create_process(
    pid: u32,
    module_path: String,
    function_args: &[String],
    process_table: ProcessTable,
) {
    let table = Arc::clone(&process_table);

    let engine = Engine::default();
    if let Err(err) = Module::from_file(&engine, module_path.clone()) {
        println!("Invalid module path {:?}", err);
    } else {
        let module = Module::from_file(&engine, module_path.clone()).unwrap();
        let mut linker = Linker::new(&engine);
        wasi_common::sync::add_to_linker(&mut linker, |t| t).unwrap();
        // let pre = linker.instantiate_pre(&module).unwrap();

        let dir = Dir::open_ambient_dir(
            "/Users/cristianaandrei/facultate/uniprocess-os/src",
            ambient_authority(),
        )
        .unwrap();
        let working_directory = "/Users/cristianaandrei/facultate/uniprocess-os/src";
        env::set_current_dir(Path::new(working_directory)).unwrap();

        let proc_table = table.lock().unwrap();
        let process_vec: Vec<&PCB> = proc_table.values().collect();

        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .preopened_dir(dir.try_clone().unwrap(), ".")
            .unwrap()
            .inherit_args()
            .unwrap()
            .env("PWD", working_directory)
            .unwrap()
            .env(
                "PROCESS_TABLE",
                &serde_json::to_string(&process_vec).unwrap(),
            )
            .unwrap()
            .args(function_args) //uncomment pentru a pasa argumente functiilor mkdir, touch etc
            .unwrap()
            .build();

        drop(proc_table);
        let mut store = Store::new(&engine, wasi);
        let linking1 = linker.instantiate(&mut store, &module).unwrap();

        let run = linking1
            .get_typed_func::<(), ()>(&mut store, "_start")
            .unwrap();

        let module_path_clone = module_path.clone();
        let _ = thread::spawn(move || {
            let mut first_table = table.lock().unwrap();
            first_table.insert(
                pid,
                PCB {
                    pid,
                    name: module_path_clone,
                    started_at: Local::now(),
                    status: "Running".to_string(),
                    // memory: linking1
                    //     .clone()
                    //     .get_memory(store, "memory")
                    //     .unwrap()
                    //     .size(&store) as usize
                    //     * 64
                    //     * 1024,
                },
            );

            drop(first_table);
            run.call(&mut store, ()).unwrap();

            let mut another_table = table.lock().unwrap();
            if let Some(info) = another_table.get_mut(&pid.clone()) {
                info.status = "Exited".to_string();
            }
            drop(another_table)
        })
        .join();
    }
}
