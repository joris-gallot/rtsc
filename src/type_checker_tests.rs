#[cfg(test)]
mod tests {
  use crate::ast::{Positioned, Program};
  use crate::lexer::Lexer;
  use crate::parser::Parser;
  use crate::type_checker::TypeChecker;
  use std::panic::{AssertUnwindSafe, catch_unwind};

  // Helper function to parse a program string
  fn parse_program(input: &str) -> Program {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.collect_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_program()
  }

  // Helper function to check a program with the type checker
  // Returns the panic message if it panics, None otherwise
  fn type_check(program: &Program) -> Option<String> {
    let result = catch_unwind(AssertUnwindSafe(|| {
      let mut checker = TypeChecker::new();
      checker.check_program(program);
    }));

    match result {
      Ok(_) => None,
      Err(e) => {
        if let Some(msg) = e.downcast_ref::<String>() {
          Some(msg.clone())
        } else if let Some(msg) = e.downcast_ref::<&str>() {
          Some(msg.to_string())
        } else {
          Some("Unknown panic".to_string())
        }
      }
    }
  }

  // Helper function to check a string program with the type checker
  // Returns the panic message if it panics, None otherwise
  fn type_check_str(input: &str) -> Option<String> {
    let program = parse_program(input);
    type_check(&program)
  }

  #[test]
  fn test_simple_number_assignment() {
    assert_eq!(type_check_str("let x: number = 42;"), None);
  }

  #[test]
  fn test_simple_string_assignment() {
    assert_eq!(type_check_str("let s: string = \"hello\";"), None);
  }

  #[test]
  fn test_number_arithmetic() {
    assert_eq!(type_check_str("let x: number = 10 + 20;"), None);
    assert_eq!(type_check_str("let x: number = 10 - 20;"), None);
    assert_eq!(type_check_str("let x: number = 10 * 20;"), None);
    assert_eq!(type_check_str("let x: number = 10 / 20;"), None);
  }

  #[test]
  fn test_string_concatenation() {
    assert_eq!(
      type_check_str("let s: string = \"hello\" + \"world\";"),
      None
    );
  }

  #[test]
  fn test_string_invalid_operation() {
    let result = type_check_str("let s: string = \"hello\" - \"world\";");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "Type error: '-' can only be used for number operations, not with strings"
    );
  }

  #[test]
  fn test_type_mismatch_number_string() {
    let result = type_check_str("let x: number = \"hello\";");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "1:5 - Type mismatch for 'x': expected Number"
    );
  }

  #[test]
  fn test_type_mismatch_string_number() {
    let result = type_check_str("let s: string = 42;");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "1:5 - Type mismatch for 's': expected String"
    );
  }

  #[test]
  fn test_type_mismatch_in_binary_expr() {
    let result = type_check_str("let x: number = 42 + \"hello\";");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "Type error: Cannot apply '+' operation between different types (Number and String)"
    );
  }

  #[test]
  fn test_variable_reference() {
    assert_eq!(
      type_check_str("let x: number = 42; let y: number = x;"),
      None
    );
  }

  #[test]
  fn test_variable_reference_wrong_type() {
    let result = type_check_str("let x: string = \"hello\"; let y: number = x;");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "1:30 - Type mismatch for 'y': expected Number"
    );
  }

  #[test]
  fn test_complex_expression() {
    assert_eq!(
      type_check_str("let x: number = 5; let y: number = 10; let z: number = (x + y) * (y - x);"),
      None
    );
  }

  #[test]
  fn test_string_variable_reference() {
    assert_eq!(
      type_check_str(
        "let s1: string = \"hello\"; let s2: string = \"world\"; let s3: string = s1 + s2;"
      ),
      None
    );
  }

  #[test]
  fn test_undefined_variable() {
    let result = type_check_str("let x: number = undefined;");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "1:5 - Type mismatch for 'x': expected Number"
    );
  }

  #[test]
  fn test_mixed_types_in_expression() {
    let result =
      type_check_str("let x: number = 5; let s: string = \"hello\"; let result: number = x + s;");
    assert!(result.is_some());
    assert_eq!(
      result.unwrap(),
      "Type error: Cannot apply '+' operation between different types (Number and String)"
    );
  }
}
