mod ast;
use ast::lexer::Token;

fn main() {
    let input: &str = "7 * 9 + (6 - 9)";
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("{:?}", tokens);
}
