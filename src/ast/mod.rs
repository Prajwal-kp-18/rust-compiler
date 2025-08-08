pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<ASTStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }
    
    pub fn visit(&self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printor = ASTPrintor { indent: 0 };
        self.visit(&mut printor);
    }
}

pub trait ASTVisitor {
    fn do_visit_statement(&mut self, statement: &ASTStatement) {
         match &statement.kind {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }
    fn visit_statement(&mut self, statement: &ASTStatement){
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self, expression: &ASTExpression) {
        match &expression.kind {
            ASTExpressionKind::Number(number) => self.visit_number(number),
        }
    }
    fn visit_expression(&mut self, expression: &ASTExpression){
        self.do_visit_expression(expression);
    }
    fn visit_number(&self, number: &ASTNumberExpression);
}

pub struct ASTPrintor{
    indent: usize,
}
const LEVEL_INDENT: usize = 2;

impl ASTVisitor for ASTPrintor {

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement");
        self.indent +=LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -=LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("Expression");
        self.indent +=LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -=LEVEL_INDENT;
    }

    fn visit_number(&self, number: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}", number.number));
    }


}

impl ASTPrintor {
    fn print_with_indent(&self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
}

pub enum  ASTStatementKind{
    Expression(ASTExpression),
}

pub struct ASTStatement {
    kind: ASTStatementKind,
} 

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        ASTStatement { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }
}

pub enum ASTExpressionKind {
    Number(ASTNumberExpression),
        
}

pub struct ASTNumberExpression {
    number: i64,
}
pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        ASTExpression { kind }
    }

    pub fn number(number: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number }))
    }
}