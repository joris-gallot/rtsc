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

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken {
  pub token: Token,
  pub line: usize,
  pub column: usize,
}

pub struct Lexer {
  input: Vec<char>,
  position: usize,
  line: usize,
  column: usize,
}

impl Lexer {
  pub fn new(input: &str) -> Self {
    Self {
      input: input.chars().collect(),
      position: 0,
      line: 1,
      column: 1,
    }
  }

  pub fn collect_tokens(&mut self) -> Vec<SpannedToken> {
    let mut tokens = Vec::new();
    loop {
      let next_token = self.next_token();
      if next_token.token == Token::EOF {
        break;
      }
      tokens.push(next_token);
    }
    tokens
  }

  fn peek(&self) -> Option<char> {
    self.input.get(self.position).cloned()
  }

  fn advance(&mut self) {
    if let Some(c) = self.peek() {
      if c == '\n' {
        self.line += 1;
        self.column = 1;
      } else {
        self.column += 1;
      }
    }
    self.position += 1;
  }

  fn skip_whitespace(&mut self) {
    while matches!(self.peek(), Some(c) if c.is_whitespace()) {
      self.advance();
    }
  }

  pub fn next_token(&mut self) -> SpannedToken {
    self.skip_whitespace();
    let line = self.line;
    let column = self.column;

    let token = match self.peek() {
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
      Some(c) => {
        panic!("Unknown character: {}", c);
      }
      None => Token::EOF,
    };

    SpannedToken {
      token,
      line,
      column,
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
