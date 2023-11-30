pub type Location = std::ops::Range<usize>;

use crate::lexer::tokens::{Type, Value};

pub mod to_string;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
  pub data_section: DataSection,
  pub text_section: TextSection,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataSection {
  pub variables: Vec<Variable>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TextSection {
  pub statements: Vec<Statement>,
  pub entrypoint: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
  pub name: String,
  pub type_: Type,
  pub value: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Instruction(Instruction),
  Label(String),
}

#[derive(Clone, Debug, PartialEq)]
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
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InstructionArgument {
  Register(Register),
  Immediate(u32),
  Label(String),
  Literal(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Register {
  pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
  Immediate(i32),
  Register(Register),
  Label(String),
}
