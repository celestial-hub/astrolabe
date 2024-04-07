pub type Location = std::ops::Range<usize>;

use serde::Serialize;

use crate::lexer::tokens::{Type, Value};

pub mod to_string;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Program {
  pub data_section: DataSection,
  pub text_section: TextSection,
}

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataSection {
  pub variables: Vec<Variable>,
}

#[derive(Serialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextSection {
  pub statements: Vec<Statement>,
  pub entrypoint: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
  pub name: String,
  pub type_: Type,
  pub value: Value,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind", content = "value")]
pub enum Statement {
  Instruction(Instruction),
  Label(String),
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "kind", content = "args")]
#[serde(rename_all = "camelCase")]
pub enum Instruction {
  Li(Vec<InstructionArgument>),
  La(Vec<InstructionArgument>),
  Syscall,
  Move(Vec<InstructionArgument>),
  Jal(Vec<InstructionArgument>),
  Beq(Vec<InstructionArgument>),
  Sub(Vec<InstructionArgument>),
  Add(Vec<InstructionArgument>),
  Jr(Vec<InstructionArgument>),
  Addi(Vec<InstructionArgument>),
  Andi(Vec<InstructionArgument>),
  J(Vec<InstructionArgument>),
  Sw(Vec<InstructionArgument>),
  Lw(Vec<InstructionArgument>),
}

impl Instruction {
  pub fn new(name: String, args: Vec<InstructionArgument>) -> Self {
    match name.as_str() {
      "li" => Instruction::Li(args),
      "la" => Instruction::La(args),
      "syscall" => Instruction::Syscall,
      "move" => Instruction::Move(args),
      "jal" => Instruction::Jal(args),
      "beq" => Instruction::Beq(args),
      "sub" => Instruction::Sub(args),
      "add" => Instruction::Add(args),
      "jr" => Instruction::Jr(args),
      "addi" => Instruction::Addi(args),
      "andi" => Instruction::Andi(args),
      "j" => Instruction::J(args),
      "sw" => Instruction::Sw(args),
      "lw" => Instruction::Lw(args),
      _ => unreachable!(),
    }
  }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind", content = "value")]
pub enum InstructionArgument {
  Register(Register),
  Immediate(u32),
  Label(String),
  Literal(String),
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Register {
  pub name: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "kind", content = "value")]
pub enum Operand {
  Immediate(i32),
  Register(Register),
  Label(String),
}
