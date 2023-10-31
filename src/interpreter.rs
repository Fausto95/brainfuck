use super::tokenizer::Token;
use super::parser::ASTNode;

pub fn interpreter(ast: &[ASTNode], data: &mut [u8; 30000], ptr: &mut usize, input_chars: &mut Vec<Token>, output: &mut Vec<Token>) {
    for node in ast {
        match node {
            ASTNode::Command(token) => {
                match token {
                    '>' => *ptr += 1,
                    '<' => *ptr -= 1,
                    '+' => data[*ptr] += 1,
                    '-' => data[*ptr] -= 1,
                    '.' => output.push(data[*ptr] as char),
                    ',' => {
                        if let Some(c) = input_chars.pop() {
                            data[*ptr] = c as u8;
                        }
                    },
                    _ => {}
                }
            },
            ASTNode::Loop(body) => {
                while data[*ptr] != 0 {
                    interpreter(body, data, ptr, input_chars, output);
                }
            }
        }
    }
}