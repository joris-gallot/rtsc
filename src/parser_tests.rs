#[cfg(test)]
mod tests {
  use crate::ast::{BinaryOp, Expr, Program};
  use crate::lexer::Lexer;
  use crate::parser::Parser;

  fn parse_program(input: &str) -> Program {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.collect_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_program()
  }

  fn assert_binary_expr(
    expr: &Expr,
    expected_left: &Expr,
    expected_op: &BinaryOp,
    expected_right: &Expr,
  ) {
    match expr {
      Expr::Binary { left, op, right } => {
        assert_eq!(**left, *expected_left);
        assert_eq!(*op, *expected_op);
        assert_eq!(**right, *expected_right);
      }
      _ => panic!("Expected Binary expression, got {:?}", expr),
    }
  }

  #[test]
  fn test_parse_simple_let_number() {
    let program = parse_program("let x: number = 42;");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    assert_eq!(stmt.name, "x");
    assert_eq!(stmt.type_name, "number");

    match &stmt.value {
      Expr::Number(n) => assert_eq!(*n, 42.0),
      _ => panic!("Expected Number expression, got {:?}", stmt.value),
    }
  }

  #[test]
  fn test_parse_simple_let_identifier() {
    let program = parse_program("let y: number = x;");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    assert_eq!(stmt.name, "y");
    assert_eq!(stmt.type_name, "number");

    match &stmt.value {
      Expr::Identifier(name) => assert_eq!(name, "x"),
      _ => panic!("Expected Identifier expression, got {:?}", stmt.value),
    }
  }

  #[test]
  fn test_parse_let_with_binary_expr() {
    let program = parse_program("let result: number = 10 + 20;");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    assert_eq!(stmt.name, "result");
    assert_eq!(stmt.type_name, "number");

    assert_binary_expr(
      &stmt.value,
      &Expr::Number(10.0),
      &BinaryOp::Add,
      &Expr::Number(20.0),
    );
  }

  #[test]
  fn test_parse_complex_binary_expr() {
    let program = parse_program("let complex: number = 5 + 10 * 15;");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    assert_eq!(stmt.name, "complex");
    assert_eq!(stmt.type_name, "number");

    // Should parse as 5 + (10 * 15) due to operator precedence
    if let Expr::Binary { left, op, right } = &stmt.value {
      assert_eq!(**left, Expr::Number(5.0));
      assert_eq!(*op, BinaryOp::Add);

      assert_binary_expr(
        right,
        &Expr::Number(10.0),
        &BinaryOp::Mul,
        &Expr::Number(15.0),
      );
    } else {
      panic!("Expected Binary expression");
    }
  }

  #[test]
  fn test_parse_parenthesized_expr() {
    let program = parse_program("let paren: number = (10 + 20) * 30;");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    // Should parse as (10 + 20) * 30
    if let Expr::Binary { left, op, right } = &stmt.value {
      assert_eq!(*op, BinaryOp::Mul);
      assert_eq!(**right, Expr::Number(30.0));

      assert_binary_expr(
        left,
        &Expr::Number(10.0),
        &BinaryOp::Add,
        &Expr::Number(20.0),
      );
    } else {
      panic!("Expected Binary expression");
    }
  }

  #[test]
  fn test_parse_deep_parenthesized_expr() {
    let program = parse_program("let result: number = ((5 * (10 + 2)) + ((8 - 3) * 6));");

    assert_eq!(program.statements.len(), 1);
    let stmt = &program.statements[0];

    assert_eq!(stmt.name, "result");
    assert_eq!(stmt.type_name, "number");

    // This should parse as: ((5 * (10 + 2)) + ((8 - 3) * 6))
    if let Expr::Binary {
      left,
      op: op_outer,
      right,
    } = &stmt.value
    {
      // Check top-level: ... + ...
      assert_eq!(*op_outer, BinaryOp::Add);

      // Check left branch: (5 * (10 + 2))
      if let Expr::Binary {
        left: left_inner,
        op: op_left,
        right: right_inner,
      } = &**left
      {
        assert_eq!(*op_left, BinaryOp::Mul);
        assert_eq!(**left_inner, Expr::Number(5.0));

        // Check (10 + 2)
        assert_binary_expr(
          right_inner,
          &Expr::Number(10.0),
          &BinaryOp::Add,
          &Expr::Number(2.0),
        );
      } else {
        panic!("Expected Binary expression for left branch");
      }

      // Check right branch: ((8 - 3) * 6)
      if let Expr::Binary {
        left: left_inner,
        op: op_right,
        right: right_inner,
      } = &**right
      {
        assert_eq!(*op_right, BinaryOp::Mul);
        assert_eq!(**right_inner, Expr::Number(6.0));

        // Check (8 - 3)
        assert_binary_expr(
          left_inner,
          &Expr::Number(8.0),
          &BinaryOp::Sub,
          &Expr::Number(3.0),
        );
      } else {
        panic!("Expected Binary expression for right branch");
      }
    } else {
      panic!("Expected Binary expression at top level");
    }
  }

  #[test]
  fn test_multiple_statements() {
    let program = parse_program("let x: number = 10; let y: number = 20;");

    assert_eq!(program.statements.len(), 2);

    let stmt1 = &program.statements[0];
    assert_eq!(stmt1.name, "x");
    assert_eq!(stmt1.type_name, "number");
    match &stmt1.value {
      Expr::Number(n) => assert_eq!(*n, 10.0),
      _ => panic!("Expected Number expression"),
    }

    let stmt2 = &program.statements[1];
    assert_eq!(stmt2.name, "y");
    assert_eq!(stmt2.type_name, "number");
    match &stmt2.value {
      Expr::Number(n) => assert_eq!(*n, 20.0),
      _ => panic!("Expected Number expression"),
    }
  }

  #[test]
  fn test_complex_program() {
    let program = parse_program(
      "
            let x: number = 5;
            let y: number = 10;
            let result: number = (x + y) * (y - x);
        ",
    );

    assert_eq!(program.statements.len(), 3);

    // Check third statement with complex expression
    let stmt3 = &program.statements[2];
    assert_eq!(stmt3.name, "result");

    if let Expr::Binary { left, op, right } = &stmt3.value {
      assert_eq!(*op, BinaryOp::Mul);

      // Check (x + y)
      assert_binary_expr(
        left,
        &Expr::Identifier("x".to_string()),
        &BinaryOp::Add,
        &Expr::Identifier("y".to_string()),
      );

      // Check (y - x)
      assert_binary_expr(
        right,
        &Expr::Identifier("y".to_string()),
        &BinaryOp::Sub,
        &Expr::Identifier("x".to_string()),
      );
    } else {
      panic!("Expected Binary expression");
    }
  }

  #[test]
  #[should_panic(expected = "Type expected")]
  fn test_error_missing_type() {
    parse_program("let x = 10;");
  }

  #[test]
  #[should_panic(expected = "expected")]
  fn test_error_missing_semicolon() {
    parse_program("let x: number = 10");
  }
}
