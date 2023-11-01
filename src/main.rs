mod tokenizer;
mod parser;
mod compiler;
mod interpreter;

fn main() {
    let source = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let output = compiler::compiler(source);

    println!("Output: {:?}", output);
}
