use crate::ast::{ASTVisitor, ASTBinaryExpression, ASTNumberExpression, ASTBinaryOperatorKind};


pub struct ASTEvaluator {
    pub last_value: Option<i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_number(&mut self, number: &ASTNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, expr: &ASTBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();

        self.last_value = match expr.operator.kind {
            ASTBinaryOperatorKind::Plus => Some(left + right),
            ASTBinaryOperatorKind::Minus => Some(left - right),
            ASTBinaryOperatorKind::Multiply => Some(left * right),
            ASTBinaryOperatorKind::Divide => Some(left / right),
            _ => None,
        };
    }
}