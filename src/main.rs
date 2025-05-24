mod ast;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parser;
#[cfg(test)]
mod parser_tests;
mod type_checker;
#[cfg(test)]
mod type_checker_tests;
use lexer::Lexer;
use parser::Parser;
use type_checker::TypeChecker;

fn main() {
  let source = r#"
let x: number = 10 + 20;
let str: number = "Hello" + "World";
"#;

  let mut lexer = Lexer::new(source);

  let tokens = lexer.collect_tokens();

  let program = Parser::new(tokens.clone()).parse_program();

  println!("{:#?}", program);

  let mut checker = TypeChecker::new();
  checker.check_program(&program);
}
