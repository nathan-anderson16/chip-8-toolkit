use c8cc::{self, lexer::lex};

fn main() {
    let tokens = lex("int main() {\n    return 100;\n}");
    println!("{tokens:?}");
}
