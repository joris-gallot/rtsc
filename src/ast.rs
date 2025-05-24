#[derive(Debug, Clone, PartialEq)]
pub struct Positioned<T> {
  pub value: T,
  pub line: usize,
  pub column: usize,
}

impl<T> Positioned<T> {
  pub fn new(value: T, line: usize, column: usize) -> Self {
    Self {
      value,
      line,
      column,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  String(String),
  Number(f64),
  Identifier(String),
  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Number,
  String,
  Unknown,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
  pub name: Positioned<String>,
  pub type_name: Positioned<String>,
  pub expression: Positioned<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Program {
  pub statements: Vec<LetStatement>,
}
