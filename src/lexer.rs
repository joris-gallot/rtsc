#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Let,
  Colon,
  Equal,
  Semicolon,
  Plus,
  Minus,
  Star,
  Slash,
  LParen,
  RParen,
  Number(f64),
  Ident(String),
  Type(String),
  EOF,
}

pub struct Lexer {
  input: Vec<char>,
  position: usize,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Self {
      input: input.chars().collect(),
      position: 0,
    }
  }

  pub fn collect_tokens(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
      let token = self.next_token();
      if token == Token::EOF {
        break;
      }
      tokens.push(token);
    }
    tokens
  }

  fn peek(&self) -> Option<char> {
    self.input.get(self.position).cloned()
  }

  fn advance(&mut self) {
    self.position += 1;
  }

  fn skip_whitespace(&mut self) {
    while matches!(self.peek(), Some(c) if c.is_whitespace()) {
      self.advance();
    }
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespace();

    let current = self.peek();
    match current {
      Some(c) if c.is_ascii_alphabetic() => self.read_ident_or_keyword(),
      Some(c) if c.is_ascii_digit() => self.read_number(),
      Some('+') => {
        self.advance();
        Token::Plus
      }
      Some('-') => {
        self.advance();
        Token::Minus
      }
      Some('*') => {
        self.advance();
        Token::Star
      }
      Some('/') => {
        self.advance();
        Token::Slash
      }
      Some('(') => {
        self.advance();
        Token::LParen
      }
      Some(')') => {
        self.advance();
        Token::RParen
      }
      Some(':') => {
        self.advance();
        Token::Colon
      }
      Some(';') => {
        self.advance();
        Token::Semicolon
      }
      Some('=') => {
        self.advance();
        Token::Equal
      }
      Some(_) => {
        panic!("Unknown character");
      }
      None => Token::EOF,
    }
  }

  fn read_ident_or_keyword(&mut self) -> Token {
    let mut ident = String::new();
    while let Some(c) = self.peek() {
      if c.is_ascii_alphanumeric() || c == '_' {
        ident.push(c);
        self.advance();
      } else {
        break;
      }
    }

    match ident.as_str() {
      "let" => Token::Let,
      "number" => Token::Type("number".to_string()),
      _ => Token::Ident(ident),
    }
  }

  fn read_number(&mut self) -> Token {
    let mut num = String::new();
    while let Some(c) = self.peek() {
      if c.is_ascii_digit() || c == '.' {
        num.push(c);
        self.advance();
      } else {
        break;
      }
    }

    Token::Number(num.parse().unwrap())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
  fn test_let_binding() {
    let input = "let x: number = 10;";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Let,
        Token::Ident("x".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(10.0),
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
        Token::Ident("x".to_string()),
        Token::Ident("y".to_string()),
        Token::Ident("foo".to_string()),
        Token::Ident("bar".to_string()),
        Token::Ident("baz".to_string()),
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
        Token::Ident("letx".to_string()),
        Token::Ident("lettering".to_string()),
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
  fn test_custom_type_identifiers() {
    let input = "string boolean any void";
    let tokens = collect_tokens(input);

    assert_eq!(
      tokens,
      vec![
        Token::Ident("string".to_string()),
        Token::Ident("boolean".to_string()),
        Token::Ident("any".to_string()),
        Token::Ident("void".to_string()),
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
        Token::Ident("x".to_string()),
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
        Token::Ident("result".to_string()),
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
        Token::Ident("x".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(10.0),
        Token::Semicolon,
        Token::Let,
        Token::Ident("y".to_string()),
        Token::Colon,
        Token::Type("number".to_string()),
        Token::Equal,
        Token::Number(20.0),
        Token::Semicolon,
      ]
    );
  }
}
