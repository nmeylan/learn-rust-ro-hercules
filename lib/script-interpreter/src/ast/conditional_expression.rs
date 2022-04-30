// Generated by lib/script-interpreter/generator
// Auto generated file do not edit manually

#[derive(Clone, Debug)]
pub enum ConditionalExpression {
    ConditionalExpression0{ logical_or_expression: Box<crate::ast::logical_or_expression::LogicalOrExpression>, question_mark: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, colon: crate::token::Token, conditional_expression: Box<crate::ast::conditional_expression::ConditionalExpression> },
    LogicalOrExpression(Box<crate::ast::logical_or_expression::LogicalOrExpression>),
    
}

impl ConditionalExpression {
    pub fn build_from_conditional_expression0(logical_or_expression: Box<crate::ast::logical_or_expression::LogicalOrExpression>, question_mark: crate::token::Token, any_expression: Box<crate::ast::any_expression::AnyExpression>, colon: crate::token::Token, conditional_expression: Box<crate::ast::conditional_expression::ConditionalExpression>) -> Self {
        ConditionalExpression::ConditionalExpression0{ logical_or_expression, question_mark, any_expression, colon, conditional_expression }
    }
    pub fn build_from_logical_or_expression(logical_or_expression: Box<crate::ast::logical_or_expression::LogicalOrExpression>) -> Self {
        ConditionalExpression::LogicalOrExpression(logical_or_expression)
    }
}

impl crate::ast::expression::Expression for ConditionalExpression {
    fn accept(&self, visitor: Box<dyn crate::ast::visitor::Visitor>) {
        visitor.visit_conditional_expression(self)
    }

}
