use crate::ast::{BinaryOp, Expr, LetStatement, Program};
use crate::lexer::Token;

pub struct Parser {
  tokens: Vec<Token>,
  position: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      position: 0,
    }
  }

  fn peek(&self) -> &Token {
    self.tokens.get(self.position).unwrap_or(&Token::EOF)
  }

  fn next(&mut self) -> &Token {
    let pos = self.position;
    self.position += 1;
    self.tokens.get(pos).unwrap_or(&Token::EOF)
  }

  fn expect(&mut self, expected: &Token) {
    let tok = self.next();
    if tok != expected {
      panic!("Type expected");
    }
  }

  pub fn parse_program(&mut self) -> Program {
    let mut statements = Vec::new();

    while self.peek() != &Token::EOF {
      statements.push(self.parse_let_statement());
    }

    Program { statements }
  }

  fn parse_let_statement(&mut self) -> LetStatement {
    self.expect(&Token::Let);

    let name = match self.next() {
      Token::Identifier(n) => n.clone(),
      other => panic!("Name expected, found {:?}", other),
    };

    self.expect(&Token::Colon);

    let type_name = match self.next() {
      Token::Type(t) => t.clone(),
      other => panic!("Type expected, found {:?}", other),
    };

    self.expect(&Token::Equal);

    let value = self.parse_expression();

    self.expect(&Token::Semicolon);

    LetStatement {
      name,
      type_name,
      value,
    }
  }

  fn parse_expression(&mut self) -> Expr {
    self.parse_term()
  }

  fn parse_term(&mut self) -> Expr {
    let mut left = self.parse_factor();

    while let Token::Plus | Token::Minus = self.peek() {
      let op = match self.next() {
        Token::Plus => BinaryOp::Add,
        Token::Minus => BinaryOp::Sub,
        _ => unreachable!(),
      };

      let right = self.parse_factor();
      left = Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
      };
    }

    left
  }

  fn parse_factor(&mut self) -> Expr {
    let mut left = self.parse_primary();

    while let Token::Star | Token::Slash = self.peek() {
      let op = match self.next() {
        Token::Star => BinaryOp::Mul,
        Token::Slash => BinaryOp::Div,
        _ => unreachable!(),
      };

      let right = self.parse_primary();
      left = Expr::Binary {
        left: Box::new(left),
        op,
        right: Box::new(right),
      };
    }

    left
  }

  fn parse_primary(&mut self) -> Expr {
    match self.next() {
      Token::LParen => {
        let expr = self.parse_expression();
        self.expect(&Token::RParen);
        expr
      }
      Token::Number(n) => Expr::Number(*n),
      Token::Identifier(name) => Expr::Identifier(name.clone()),
      other => panic!("Unexpected expression: {:?}", other),
    }
  }
}
