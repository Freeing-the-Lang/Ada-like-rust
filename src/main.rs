mod ada_style;
mod runtime;
mod proofledger;

use ada_style::*;
use runtime::*;
use proofledger::*;

fn main() {
    ada_begin();

    println!("Ada-like Rust — Deterministic & Safe");

    ada_procedure("Main", || {
        let result = ada_block(|| {
            let x: i32 = 42;
            let y: i32 = 8;
            ada_return(x + y)
        });

        ada_log("Computation Result", result);
    });

    ada_end();
}
