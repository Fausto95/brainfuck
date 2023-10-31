
use crate::tokenizer::tokenizer;
use crate::parser::parser;
use crate::interpreter::interpreter;


pub fn compiler(input: &str) -> String {
    let tokens = tokenizer(input);
    let ast = parser(&tokens);

    let mut data = [0u8; 30000];
    let mut ptr = 0usize;
    let mut input_chars: Vec<char> = "Brainfuck Input:".chars().collect(); // Placeholder input, replace as necessary
    let mut output = Vec::new();

    interpreter(&ast, &mut data, &mut ptr, &mut input_chars, &mut output);

    output.iter().collect()
}
