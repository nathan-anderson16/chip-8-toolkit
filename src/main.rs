use c8cc::{self, lexer::lex, parser::parse};

fn main() {
    let tokens = lex("int main() {\n    return 100;\n}");
    println!("{tokens:?}");
    let node = parse(tokens);
    println!("{node:?}");
}
