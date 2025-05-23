#[cfg(test)]
mod tests {
  use crate::lexer::{Lexer, Token};

  fn collect_tokens(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);
    lexer.collect_tokens()
  }

  #[test]
  fn test_simple_tokens() {
    let input = "=+()-/*:;";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Equal,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::Minus,
        Token::Slash,
        Token::Star,
        Token::Colon,
        Token::Semicolon,
      ]
    );
  }

  #[test]
  fn test_let_binding_number() {
    let input = "let x: number = 10;";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(10.0),
        Token::Semicolon,
      ]
    );
  }

  #[test]
  fn test_let_binding_string() {
    let input = "let str: string = \"Hello\";";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("str".to_string()),
        Token::Colon,
        Token::Type("string".to_string()),
        Token::Equal,
        Token::String("Hello".to_string()),
        Token::Semicolon,
      ]
    );
  }

  #[test]
  fn test_numeric_literal() {
    let input = "5 10 42.5 0.25";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Number(5.0),
        Token::Number(10.0),
        Token::Number(42.5),
        Token::Number(0.25),
      ]
    );
  }

  #[test]
  fn test_identifiers() {
    let input = "x y foo bar baz";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Identifier("x".to_string()),
        Token::Identifier("y".to_string()),
        Token::Identifier("foo".to_string()),
        Token::Identifier("bar".to_string()),
        Token::Identifier("baz".to_string()),
      ]
    );
  }

  #[test]
  fn test_let_keyword() {
    let input = "let letx lettering";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("letx".to_string()),
        Token::Identifier("lettering".to_string()),
      ]
    );
  }

  #[test]
  fn test_number_type() {
    let input = "number";
    let tokens = collect_tokens(input);

    assert_eq!(tokens, vec![Token::Type("number".to_string()),]);
  }

  #[test]
  fn test_string_type() {
    let input = "string";
    let tokens = collect_tokens(input);

    assert_eq!(tokens, vec![Token::Type("string".to_string()),]);
  }

  #[test]
  fn test_custom_type_identifiers() {
    let input = "boolean any void";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Identifier("boolean".to_string()),
        Token::Identifier("any".to_string()),
        Token::Identifier("void".to_string()),
      ]
    );
  }

  #[test]
  fn test_whitespace_handling() {
    let input = "  let   x  :  number  =  10  ;  ";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(10.0),
        Token::Semicolon,
      ]
    );
  }

  #[test]
  fn test_complex_expression() {
    let input = "let result: number = (10 + 20) * (30 - 5);";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("result".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::LParen,
        Token::Number(10.0),
        Token::Plus,
        Token::Number(20.0),
        Token::RParen,
        Token::Star,
        Token::LParen,
        Token::Number(30.0),
        Token::Minus,
        Token::Number(5.0),
        Token::RParen,
        Token::Semicolon,
      ]
    );
  }

  #[test]
  fn test_multiple_statements() {
    let input = "let x: number = 10; let y: number = 20;";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(10.0),
        Token::Semicolon,
        Token::Let,
        Token::Identifier("y".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(20.0),
        Token::Semicolon,
      ]
    );
  }
}
