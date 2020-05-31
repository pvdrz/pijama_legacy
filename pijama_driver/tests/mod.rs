extern crate pijama_driver;

use std::{panic, sync::mpsc, thread, time::Duration};

use pijama_core::machine::{arithmetic::CheckedArithmetic, env::Env, MachineBuilder};
use pijama_driver::{run_with_machine, LangResult};

mod ast;
mod eval;
mod parse;
mod type_check;
mod util;

fn machine_builder(output: &mut Vec<u8>) -> MachineBuilder<&mut Vec<u8>, CheckedArithmetic> {
    MachineBuilder::default()
        .with_arithmetic(CheckedArithmetic)
        .with_env(Env::new(output))
}

fn run(input: &str) -> LangResult<String> {
    let mut output = Vec::default();
    run_with_machine(input, machine_builder(&mut output).build())?;
    Ok(String::from_utf8(output).unwrap())
}

fn panic_after<T, F>(d: Duration, f: F) -> T
where
    T: Send + 'static,
    F: FnOnce() -> T,
    F: Send + 'static,
    F: std::panic::UnwindSafe,
{
    let (done_tx, done_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let result = panic::catch_unwind(f);
        done_tx.send(()).expect("Unable to send completion signal");
        result.unwrap_or_else(|e| panic!("{}", e.downcast_ref::<String>().unwrap()))
    });

    match done_rx.recv_timeout(d) {
        Ok(_) => handle
            .join()
            .unwrap_or_else(|e| panic!("Thread panicked {}", e.downcast_ref::<String>().unwrap())),
        Err(_) => panic!("Thread took too long"),
    }
}
