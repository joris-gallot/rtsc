#[derive(Debug, PartialEq)]
pub enum Expr {
  Number(f64),
  Identifier(String),
  Binary {
    left: Box<Expr>,
    op: BinaryOp,
    right: Box<Expr>,
  },
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
  pub name: String,
  pub type_name: String,
  pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct Program {
  pub statements: Vec<LetStatement>,
}
