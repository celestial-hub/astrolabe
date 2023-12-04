use celestial_hub_astrolabe::{lexer::Lexer, parser::Parser};

fn main() -> color_eyre::Result<(), Box<dyn std::error::Error>> {
  color_eyre::install()?;
  let filepath = std::env::args().nth(1).expect("missing filepath");

  let source_code = std::fs::read_to_string(filepath.clone())?;
  let lexer = Lexer::new(&source_code[..], &filepath);

  println!("Tokens: {lexer}");

  let ast = Parser::new().parse(lexer)?;

  println!("AST: {:#?}", ast);

  Ok(())
}
