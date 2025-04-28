use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let mut buf = String::new();
    let mut f = File::open(&args[1]).expect("failed to open file");
    f.read_to_string(&mut buf).expect("failed to read file");
    disassemble(&buf);
}

fn disassemble(text: &str) {
    println!("{text}");
}
