mod lexer;

use lexer::Lexer;

fn main() {
  let source = "let x: number = 10 + 20;";
  let mut lexer = Lexer::new(source);

  let mut tokens = Vec::new();
  loop {
    let tok = lexer.next_token();
    if tok == lexer::Token::EOF {
      break;
    }
    tokens.push(tok);
  }

  println!("{:#?}", tokens);
}
