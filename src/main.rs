use std::{env, process::exit};

use c8rs::{
    init::{init, set_rom_path},
    run::run,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file>", args[0]);
        exit(0);
    }

    set_rom_path(args[1].clone().leak()); // TODO: Better way to do this?

    init();
    run();
}
