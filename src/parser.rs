use super::tokenizer::Token;

pub enum ASTNode {
    Command(Token),
    Loop(Vec<ASTNode>),
}

pub fn parser(tokens: &[Token]) -> Vec<ASTNode> {
    let mut current = 0;

    fn traverse(tokens: &[Token], current: &mut usize) -> ASTNode {
        let token = tokens[*current];

        match token {
            '>' | '<' | '+' | '-' | '.' | ',' => {
                *current += 1;
                ASTNode::Command(token)
            },
            '[' => {
                *current += 1;
                let mut body = Vec::new();
                while tokens[*current] != ']' {
                    body.push(traverse(tokens, current));
                }
                *current += 1;
                ASTNode::Loop(body)
            },
            _ => panic!("Unexpected token: {}", token),
        }
    }

    let mut ast = Vec::new();

    while current < tokens.len() {
        ast.push(traverse(&tokens, &mut current));
    }

    ast
}
