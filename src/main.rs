mod ast;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parser;
#[cfg(test)]
mod parser_tests;
use lexer::Lexer;
use parser::Parser;

fn main() {
  let source = "let x: number = 10 + 20;";
  let mut lexer = Lexer::new(source);

  let tokens = lexer.collect_tokens();

  let program = Parser::new(tokens.clone()).parse_program();

  println!("{:#?}", program);
}
