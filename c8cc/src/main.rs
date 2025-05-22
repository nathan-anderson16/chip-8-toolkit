use std::{env, fs::File, io::Write, path::Path, process::Command};

use c8cc::{self, compile::compile, lexer::lex, parser::parse};

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2, "usage: {} file", args[0]);

    let if_path = Path::new(&args[1]);

    let tokens = lex("int main() {\n    return 0;\n}");
    // println!("{tokens:?}");
    let program = parse(tokens);
    println!("{program:?}");
    let instructions = compile(&program);

    let of_path = if_path.with_extension("asm");
    let mut of = File::create(of_path.clone()).unwrap();
    for instruction in instructions {
        of.write_all(instruction.asm().as_bytes()).unwrap();
        of.write_all(b"\n").unwrap();
    }

    // Convert asm to bytecode
    env::set_current_dir("../c8asm").expect("missing crate: c8asm");
    let command = Command::new("cargo")
        .arg("run")
        .arg(format!(
            "../c8cc/{}",
            of_path.file_name().unwrap().to_str().unwrap()
        ))
        .arg(format!(
            "../c8cc/{}",
            of_path
                .with_extension("c8")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        ))
        .output();
    println!("{command:?}");
}
