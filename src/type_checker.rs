use crate::ast::{BinaryOp, Expr, LetStatement, Program, Type};
use std::collections::HashMap;

pub struct TypeChecker {
  env: HashMap<String, Type>,
}

impl TypeChecker {
  pub fn new() -> Self {
    TypeChecker {
      env: HashMap::new(),
    }
  }

  pub fn check_program(&mut self, program: &Program) {
    for stmt in &program.statements {
      self.check_let(stmt);
    }
  }

  fn check_let(&mut self, stmt: &LetStatement) {
    let expected_type = match stmt.type_name.as_str() {
      "number" => Type::Number,
      "string" => Type::String,
      _ => Type::Unknown,
    };

    let actual_type = self.check_expr(&stmt.value);

    if expected_type != actual_type {
      panic!(
        "Type mismatch for '{}': expected {:?}, found {:?}",
        stmt.name, expected_type, actual_type
      );
    }

    self.env.insert(stmt.name.clone(), expected_type);
  }

  fn check_expr(&self, expr: &Expr) -> Type {
    match expr {
      Expr::Number(_) => Type::Number,
      Expr::String(_) => Type::String,
      Expr::Identifier(name) => self.env.get(name).cloned().unwrap_or(Type::Unknown),
      Expr::Binary { left, op, right } => {
        let left_type = self.check_expr(left);
        let right_type = self.check_expr(right);

        if left_type == Type::Number && right_type == Type::Number {
          Type::Number
        } else if left_type == Type::String && right_type == Type::String {
          if *op == BinaryOp::Add {
            Type::String
          } else {
            panic!(
              "Type error: '{:?}' can only be used for string concatenation",
              op
            );
          }
        } else {
          Type::Unknown
        }
      }
    }
  }
}
