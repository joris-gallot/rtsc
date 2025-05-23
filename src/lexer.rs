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
  String(String),
  Identifier(String),
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
      Some('"') => self.read_string(),
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
        panic!("Unknown character: {}", current.unwrap());
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
      "string" => Token::Type("string".to_string()),
      _ => Token::Identifier(ident),
    }
  }

  fn read_string(&mut self) -> Token {
    self.advance();
    let mut string = String::new();
    while let Some(c) = self.peek() {
      if c == '"' {
        self.advance();
        break;
      }
      string.push(c);
      self.advance();
    }

    Token::String(string)
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
