mod lexer;

use lexer::Lexer;

fn main() {
  let source = "let x: number = 10 + 20;";
  let mut lexer = Lexer::new(source);
  
  let tokens = lexer.collect_tokens();

  println!("{:#?}", tokens);
}
