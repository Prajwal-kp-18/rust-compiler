mod ast;
use ast::lexer::Token;
use ast::Ast;
use ast::parser::Parser;

fn main() {
    let input: &str = "0";
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }
    println!("Tokens: {:?}", tokens);


    let mut ast: Ast = Ast::new();
    let mut parser = Parser::from_tokens(tokens);
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();
}
