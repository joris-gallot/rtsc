#[cfg(test)]
mod tests {
  use crate::ast::*;
  use crate::js_emitter::JsEmitter;
  use crate::lexer::Lexer;
  use crate::parser::Parser;

  fn parse_program(input: &str) -> Program {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.collect_tokens();
    let mut parser = Parser::new(tokens);
    parser.parse_program()
  }

  fn generate_js(input: &str) -> String {
    let program = parse_program(input);
    let generator = JsEmitter::new();
    generator.generate(&program)
  }

  #[test]
  fn test_simple_number_assignment() {
    let js = generate_js("let x: number = 42;");
    assert_eq!(js, "let x = 42;\n");
  }

  #[test]
  fn test_simple_string_assignment() {
    let js = generate_js("let s: string = \"hello\";");
    assert_eq!(js, "let s = \"hello\";\n");
  }

  #[test]
  fn test_variable_reference() {
    let js = generate_js("let x: number = 42; let y: number = x;");
    assert_eq!(js, "let x = 42;\nlet y = x;\n");
  }

  #[test]
  fn test_binary_expression_addition() {
    let js = generate_js("let sum: number = 10 + 20;");
    assert_eq!(js, "let sum = (10 + 20);\n");
  }

  #[test]
  fn test_binary_expression_subtraction() {
    let js = generate_js("let diff: number = 20 - 10;");
    assert_eq!(js, "let diff = (20 - 10);\n");
  }

  #[test]
  fn test_binary_expression_multiplication() {
    let js = generate_js("let product: number = 5 * 6;");
    assert_eq!(js, "let product = (5 * 6);\n");
  }

  #[test]
  fn test_binary_expression_division() {
    let js = generate_js("let quotient: number = 20 / 4;");
    assert_eq!(js, "let quotient = (20 / 4);\n");
  }

  #[test]
  fn test_string_concatenation() {
    let js = generate_js("let greeting: string = \"Hello, \" + \"World!\";");
    assert_eq!(js, "let greeting = (\"Hello, \" + \"World!\");\n");
  }

  #[test]
  fn test_complex_expression() {
    let js = generate_js("let result: number = (10 + 20) * (30 - 5);");
    assert_eq!(js, "let result = ((10 + 20) * (30 - 5));\n");
  }

  #[test]
  fn test_nested_expressions() {
    let js = generate_js("let complex: number = 5 + 10 * 15;");
    assert_eq!(js, "let complex = (5 + (10 * 15));\n");
  }

  #[test]
  fn test_multiple_statements() {
    let js = generate_js("let x: number = 10; let y: number = 20; let z: number = x + y;");
    assert_eq!(js, "let x = 10;\nlet y = 20;\nlet z = (x + y);\n");
  }

  #[test]
  fn test_variable_references_in_expressions() {
    let js =
      generate_js("let a: number = 5; let b: number = 10; let result: number = (a + b) * (b - a);");
    assert_eq!(
      js,
      "let a = 5;\nlet b = 10;\nlet result = ((a + b) * (b - a));\n"
    );
  }

  #[test]
  fn test_string_variables_concatenation() {
    let js = generate_js(
      "let s1: string = \"Hello\"; let s2: string = \"World\"; let greeting: string = s1 + \", \" + s2 + \"!\";",
    );
    assert_eq!(
      js,
      "let s1 = \"Hello\";\nlet s2 = \"World\";\nlet greeting = (((s1 + \", \") + s2) + \"!\");\n"
    );
  }
}
