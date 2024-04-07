use logos::Logos;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Type {
  Asciiz,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Value {
  String(String),
}

pub fn handle_type(lex: &mut logos::Lexer<Token>) -> Type {
  match lex.slice() {
    ".asciiz" => Type::Asciiz,
    _ => unreachable!(),
  }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r";.*")]
pub enum Token {
  // Instructions
  #[regex("li|la|syscall|move|jal|beq|sub|add|jr|addi|andi|j|sw", |lex| lex.slice().to_string())]
  Instruction(String),

  // Sections
  #[token(".data")]
  DataSection,

  #[token(".text")]
  TextSection,

  #[token(".global")]
  GlobalSection,

  // Types
  #[token(".asciiz", handle_type)]
  Type(Type),

  // Registers
  #[regex("\\$[a-z0-9]+", |lex| lex.slice().to_string())]
  Register(String),

  // Labels and identifiers
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*:", |lex| lex.slice()[..lex.slice().len() - 1].to_string())]
  Label(String),

  // Identifiers
  #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
  Identifier(String),

  // Numeric constants
  // Allow literal numbers, negative numbers, and hex numbers
  #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
  #[regex("0[xX][0-9a-fA-F]+", |lex| u32::from_str_radix(&lex.slice()[2..], 16).ok())]
  Number(u32),

  // Strings for .asciiz, ignore the quotes
  #[regex("\"[^\"]*\"", |lex| lex.slice()[1..lex.slice().len() - 1].to_string())]
  String(String),

  // Punctuation
  #[token("[")]
  OpenBracket,
  #[token("]")]
  CloseBracket,
  #[token("(")]
  OpenParen,
  #[token(")")]
  CloseParen,
  #[token(",")]
  Comma,
}
