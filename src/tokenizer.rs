pub type Token = char;

pub fn tokenizer(input: &str) -> Vec<Token> {
    input.chars()
         .filter(|&c| ['>', '<', '+', '-', '.', ',', '[', ']'].contains(&c))
         .collect()
}
