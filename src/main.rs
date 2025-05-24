mod ast;
mod js_emitter;
mod lexer;
#[cfg(test)]
mod lexer_tests;
mod parser;
use js_emitter::JsEmitter;
#[cfg(test)]
mod js_emitter_tests;
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
let str: string = "Hello" + "World";
"#;

  let mut lexer = Lexer::new(source);

  let tokens = lexer.collect_tokens();

  let program = Parser::new(tokens.clone()).parse_program();

  let mut checker = TypeChecker::new();
  checker.check_program(&program);

  let generator = JsEmitter::new();
  let js_code = generator.generate(&program);
  println!("JS code:\n{}", js_code);
}
