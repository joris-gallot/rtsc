use crate::ast::{BinaryOp, Expr, LetStatement, Positioned, Program};
use crate::lexer::{SpannedToken, Token};

pub struct Parser {
  tokens: Vec<SpannedToken>,
  position: usize,
}

impl Parser {
  pub fn new(tokens: Vec<SpannedToken>) -> Self {
    Parser {
      tokens,
      position: 0,
    }
  }

  fn peek(&self) -> &SpannedToken {
    self.tokens.get(self.position).unwrap_or(&SpannedToken {
      token: Token::EOF,
      line: 0,
      column: 0,
    })
  }

  fn next(&mut self) -> &SpannedToken {
    let pos = self.position;
    self.position += 1;
    self.tokens.get(pos).unwrap_or(&SpannedToken {
      token: Token::EOF,
      line: 0,
      column: 0,
    })
  }

  fn expect(&mut self, expected: &Token) {
    let tok = self.next();
    if &tok.token != expected {
      panic!("Expected: {:?}", expected);
    }
  }

  pub fn parse_program(&mut self) -> Program {
    let mut statements = Vec::new();

    while self.peek().token != Token::EOF {
      statements.push(self.parse_let_statement());
    }

    Program { statements }
  }

  fn parse_let_statement(&mut self) -> LetStatement {
    self.expect(&Token::Let);

    // Parse the identifier (name) with position
    let name_token = self.next();
    let name = match &name_token.token {
      Token::Identifier(n) => Positioned::new(n.clone(), name_token.line, name_token.column),
      other => panic!("Expected identifier name, found {:?}", other),
    };

    self.expect(&Token::Colon);

    // Parse the type with position
    let type_token = self.next();
    let type_name = match &type_token.token {
      Token::Type(t) => Positioned::new(t.clone(), type_token.line, type_token.column),
      other => panic!("Expected type annotation, found {:?}", other),
    };

    self.expect(&Token::Equal);

    let value_token = self.peek().clone();
    let expr_position = Positioned::new(
      self.parse_expression(),
      value_token.line,
      value_token.column,
    );

    self.expect(&Token::Semicolon);

    LetStatement {
      name,
      type_name,
      expression: expr_position,
    }
  }

  fn parse_expression(&mut self) -> Expr {
    self.parse_term()
  }

  fn parse_term(&mut self) -> Expr {
    let mut left = self.parse_factor();

    while matches!(&self.peek().token, Token::Plus | Token::Minus) {
      let op = match &self.next().token {
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

    while matches!(&self.peek().token, Token::Star | Token::Slash) {
      let op = match &self.next().token {
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
    let token = self.next();
    match &token.token {
      Token::LParen => {
        let expr = self.parse_expression();
        self.expect(&Token::RParen);
        expr
      }
      Token::Number(n) => Expr::Number(*n),
      Token::String(s) => Expr::String(s.clone()),
      Token::Identifier(name) => Expr::Identifier(name.clone()),
      other => panic!("Expected expression, found unexpected token: {:?}", other),
    }
  }
}
