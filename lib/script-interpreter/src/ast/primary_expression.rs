// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum PrimaryExpression {
    Identifier(crate::token::Token),
    Number(crate::token::Token),
    String(crate::token::Token),
    PrimaryExpression3{ left_paren: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, right_paren: crate::token::Token },
    
}

impl PrimaryExpression {
    pub fn build_from_identifier(identifier: crate::token::Token) -> Self {
        PrimaryExpression::Identifier(identifier)
    }
    pub fn build_from_number(number: crate::token::Token) -> Self {
        PrimaryExpression::Number(number)
    }
    pub fn build_from_string(string: crate::token::Token) -> Self {
        PrimaryExpression::String(string)
    }
    pub fn build_from_primary_expression3(left_paren: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, right_paren: crate::token::Token) -> Self {
        PrimaryExpression::PrimaryExpression3{ left_paren, any_expression, right_paren }
    }
}

impl crate::ast::expression::Expression for PrimaryExpression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_primary_expression(self)
    }

}
