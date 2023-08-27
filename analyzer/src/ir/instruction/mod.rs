pub mod assign;
pub mod binary;
pub mod block;
pub mod call;
pub mod class;
pub mod function;
pub mod ir_if;
pub mod ir_return;
pub mod ir_while;
pub mod literal;
pub mod logical;
pub mod unary;
pub mod variable;
pub mod ir_println;
pub mod ternary;

use enums::data_type::DataType;

use self::{
  binary::IRBinary, block::IRBlock, literal::IRLiteral, unary::IRUnary, variable::IRVariable,
  logical::IRLogical, ir_if::IRIf, ir_while::IRWhile, function::IRFunction, call::IRCall,
  class::IRClass, assign::IRAssign, ir_return::IRReturn, ternary::IRTernary,
};

#[derive(Debug, Clone)]
pub enum IRInstruction {
  Binary(IRBinary),
  Block(IRBlock),
  Literal(IRLiteral),
  Unary(IRUnary),
  Variable(IRVariable),
  Logical(IRLogical),
  If(IRIf),
  While(IRWhile),
  Function(IRFunction),
  Call(IRCall),
  Return(IRReturn),
  Assign(IRAssign),
  Class(IRClass),
  Ternary(IRTernary),
}
