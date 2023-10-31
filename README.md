
A Brainfuck interpreter written in Rust

Here's the project structure: 

- [Tokenizer](./src/tokenizer.rs): Transforms the source code into an array of `chars` 
- [Parser](./src/parser.rs): Transforms the tokenized source code into an AST(Abstract Syntax Tree)
- [Compiler (Interpreter)](./src/compiler.rs): Traverses the ast and does all the interpretation of the program

## How the hell brainfuck works?