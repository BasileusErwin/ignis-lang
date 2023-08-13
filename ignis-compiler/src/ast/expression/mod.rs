use self::{
  binary::Binary, grouping::Grouping, literal::Literal, unary::Unary, variable::VariableExpression,
  logical::Logical, assign::Assign,
};

use super::{
  lexer::{token_type::TokenType, token::Token},
  visitor::Visitor,
};

pub mod assign;
pub mod binary;
pub mod grouping;
pub mod literal;
pub mod logical;
pub mod unary;
pub mod variable;

#[derive(Debug, PartialEq)]
pub enum Expression {
  Binary(Binary),
  Grouping(Grouping),
  Literal(Literal),
  Unary(Unary),
  Variable(VariableExpression),
  Assign(Assign),
  Logical(Logical),
}

impl Expression {
  pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
    match self {
      Expression::Grouping(grouping) => visitor.visit_grouping_expression(grouping),
      Expression::Binary(binary) => visitor.visit_binary_expression(binary),
      Expression::Literal(literal) => visitor.visit_literal_expression(literal),
      Expression::Unary(unary) => visitor.visit_unary_expression(unary),
      Expression::Variable(variable) => visitor.visit_variable_expressin(variable),
      Expression::Assign(assign) => visitor.visit_assign_expression(assign),
      Expression::Logical(logical) => visitor.visit_logical_expression(logical),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Expression::Binary(Binary {
        left,
        operator,
        right,
        data_type,
      }) => format!(
        "({} {} {}): {}",
        operator.span.literal,
        left.to_string(),
        right.to_string(),
        data_type.to_string(),
      ),
      Expression::Grouping(Grouping { expression }) => {
        format!("(group {})", (*expression).to_string())
      }
      Expression::Literal(Literal { value }) => format!("{}", value.to_string()),
      Expression::Unary(Unary {
        operator,
        right,
        data_type,
      }) => {
        let operator_str = operator.span.literal.clone();
        let right_str = (*right).to_string();
        format!(
          "({} {}): {}",
          operator_str,
          right_str,
          data_type.to_string()
        )
      }
      Expression::Variable(VariableExpression { name, data_type }) => {
        format!("{:?}: {:?}", name, data_type)
      }
      Expression::Assign(Assign { name, value, .. }) => {
        format!("{} = {}", name.span.literal, value.to_string())
      }
      Expression::Logical(Logical {
        left,
        operator,
        right,
        ..
      }) => format!(
        "({} {} {})",
        left.to_string(),
        operator.span.literal,
        right.to_string()
      ),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
  Int(i64),
  Double(f64),
  Char(char),
  String(String),
  Boolean(bool),
  None,
}

impl LiteralValue {
  pub fn to_string(&self) -> String {
    match self {
      LiteralValue::Boolean(x) => x.to_string(),
      LiteralValue::None => "null".to_string(),
      LiteralValue::Double(x) => x.to_string(),
      LiteralValue::Int(x) => x.to_string(),
      LiteralValue::String(x) => x.clone(),
      LiteralValue::Char(x) => x.to_string(),
    }
  }

  pub fn from_token(token: Token) -> Self {
    match token.kind {
      TokenType::Null => Self::None,
      TokenType::Int => Self::Int(token.span.literal.parse().unwrap()),
      TokenType::Double => Self::Double(token.span.literal.parse().unwrap()),
      TokenType::String => Self::String(token.span.literal),
      TokenType::False | TokenType::True => Self::Boolean(token.span.literal.parse().unwrap()),
      _ => Self::None,
    }
  }
}
