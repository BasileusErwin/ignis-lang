use crate::{
  expression::{
    binary::Binary, literal::Literal, unary::Unary, grouping::Grouping,
    variable::VariableExpression, assign::Assign, logical::Logical, ternary::Ternary, call::Call,
  },
  statement::{
    expression::ExpressionStatement, variable::Variable, if_statement::IfStatement, block::Block,
    while_statement::WhileStatement, function::FunctionStatement, return_statement::Return,
  },
};

pub trait Visitor<R> {
  // Expression
  fn visit_binary_expression(&mut self, expression: &Binary) -> R;
  fn visit_grouping_expression(&mut self, expression: &Grouping) -> R;
  fn visit_literal_expression(&mut self, expression: &Literal) -> R;
  fn visit_unary_expression(&mut self, expression: &Unary) -> R;
  fn visit_variable_expression(&mut self, variable: &VariableExpression) -> R;
  fn visit_assign_expression(&mut self, expression: &Assign) -> R;
  fn visit_logical_expression(&mut self, expression: &Logical) -> R;
  fn visit_ternary_expression(&mut self, expression: &Ternary) -> R;
  fn visit_call_expression(&mut self, expression: &Call) -> R;

  // Statements
  fn visit_expression_statement(&mut self, statement: &ExpressionStatement) -> R;
  fn visit_variable_statement(&mut self, variable: &Variable) -> R;
  fn visit_block(&mut self, block: &Block) -> R;
  fn visit_if_statement(&mut self, statement: &IfStatement) -> R;
  fn visit_while_statement(&mut self, statement: &WhileStatement) -> R;
  fn visit_function_statement(&mut self, statement: &FunctionStatement) -> R;
  fn visit_return_statement(&mut self, statement: &Return) -> R;
}
