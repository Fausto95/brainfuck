
A Brainfuck interpreter written in Rust

Here's the project structure: 

- [Tokenizer](./src/tokenizer.rs): Transforms the source code into an array of `chars` 
- [Parser](./src/parser.rs): Transforms the tokenized source code into an AST(Abstract Syntax Tree)
- [Compiler (Interpreter)](./src/compiler.rs): Traverses the ast and does all the interpretation of the program

## How the hell brainfuck works?

To get a good understanding of how it works, I recommend watching these videos:

- [BrainFuck in 100 seconds](https://youtu.be/hdHjjBS4cs8?si=vpgj4ev7vA3TkThR)
- [How Brainfuck Works](https://youtu.be/-3C200nCwpk?si=I937lLnDiy-hB7OZ)
- [Brainfuck 42](https://www.youtube.com/watch?v=cwuPIugCHl4)
