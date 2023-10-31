mod tokenizer;
mod parser;
mod compiler;
mod interpreter;

fn main() {
    let input = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";

    println!("Output: {:?}", compiler::compiler(input));
}
