use crate::ast::*;

pub struct JsEmitter;

impl JsEmitter {
  pub fn new() -> Self {
    JsEmitter
  }

  pub fn generate(&self, program: &Program) -> String {
    let mut output = String::new();

    for stmt in &program.statements {
      let line = self.generate_statement(stmt);
      output.push_str(&line);
      output.push('\n');
    }

    output
  }

  fn generate_statement(&self, stmt: &LetStatement) -> String {
    let expr_code = Self::generate_expr(&stmt.expression.value);
    format!("let {} = {};", stmt.name.value, expr_code)
  }

  fn generate_expr(expr: &Expr) -> String {
    match expr {
      Expr::Number(n) => n.to_string(),
      Expr::String(s) => format!("\"{}\"", s),
      Expr::Identifier(name) => name.clone(),
      Expr::Binary { left, op, right } => {
        let left_code = Self::generate_expr(left);
        let right_code = Self::generate_expr(right);
        let op_str = op.to_str();
        format!("({} {} {})", left_code, op_str, right_code)
      }
    }
  }
}
