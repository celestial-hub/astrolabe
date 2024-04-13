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

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextSection {
  pub statements: Vec<Statement>,
  pub entrypoint: String,
}

impl Default for TextSection {
  fn default() -> Self {
    Self {
      statements: Default::default(),
      entrypoint: "main".into(),
    }
  }
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
  Sub(Vec<InstructionArgument>),
  Add(Vec<InstructionArgument>),
  Jr(Vec<InstructionArgument>),
  Addi(Vec<InstructionArgument>),
  Andi(Vec<InstructionArgument>),

  /// Jump to label. `j label`
  J(Vec<InstructionArgument>),

  /// Store word. `sw $t0, 0($t1)`
  Sw(Vec<InstructionArgument>),

  /// Load word. `lw $t0, $t1`
  Lw(Vec<InstructionArgument>),

  /// Set if less than. `slt $t0, $t1, $t2`
  Slt(Vec<InstructionArgument>),

  /// Set if less or equal to. `sle $t0, $t1, $t2`
  Sle(Vec<InstructionArgument>),

  /// Set if greater than. `sgt $t0, $t1, $t2`
  Sgt(Vec<InstructionArgument>),

  /// Set if greater than or equal to. `sge $t0, $t1, $t2`
  Sge(Vec<InstructionArgument>),

  /// Set if equal. `seq $t0, $t1, $t2`
  Seq(Vec<InstructionArgument>),

  /// Set if not equal. `sne $t0, $t1, $t2`
  Sne(Vec<InstructionArgument>),

  /// Branch if equal zero. `beqz $t0, label`
  Beqz(Vec<InstructionArgument>),

  /// Branch if not equal zero. `bnez $t0, label`
  Bnez(Vec<InstructionArgument>),

  /// Branch less than zero. `bltz $t0, label`
  Bltz(Vec<InstructionArgument>),

  /// Branch greater than zero. `bgtz $t0, label`
  Bgtz(Vec<InstructionArgument>),

  /// Branch less than or equal to zero. `blez $t0, label`
  Blez(Vec<InstructionArgument>),

  /// Branch greater than or equal to zero. `bgez $t0, label`
  Bgez(Vec<InstructionArgument>),

  /// Branch less than. `blt $t0, $t1, label`
  Blt(Vec<InstructionArgument>),

  /// Branch greater than. `bgt $t0, $t1, label`
  Bgt(Vec<InstructionArgument>),

  /// Branch less than or equal to. `ble $t0, $t1, label`
  Ble(Vec<InstructionArgument>),

  /// Branch greater than or equal to. `bge $t0, $t1, label`
  Bge(Vec<InstructionArgument>),

  /// Branch on equal. `beq $t0, $t1, label`
  Beq(Vec<InstructionArgument>),

  /// Branch on not equal. `bne $t0, $t1, label`
  Bne(Vec<InstructionArgument>),
}

impl Instruction {
  pub fn new(name: String, args: Vec<InstructionArgument>) -> Self {
    match name.as_str() {
      "li" => Instruction::Li(args),
      "la" => Instruction::La(args),
      "syscall" => Instruction::Syscall,
      "move" => Instruction::Move(args),
      "jal" => Instruction::Jal(args),
      "sub" => Instruction::Sub(args),
      "add" => Instruction::Add(args),
      "jr" => Instruction::Jr(args),
      "addi" => Instruction::Addi(args),
      "andi" => Instruction::Andi(args),
      "j" => Instruction::J(args),
      "sw" => Instruction::Sw(args),
      "lw" => Instruction::Lw(args),
      "slt" => Instruction::Slt(args),
      "sle" => Instruction::Sle(args),
      "sgt" => Instruction::Sgt(args),
      "sge" => Instruction::Sge(args),
      "seq" => Instruction::Seq(args),
      "sne" => Instruction::Sne(args),
      "beqz" => Instruction::Beqz(args),
      "bnez" => Instruction::Beqz(args),
      "bltz" => Instruction::Bltz(args),
      "bgtz" => Instruction::Bgtz(args),
      "blez" => Instruction::Blez(args),
      "bgez" => Instruction::Bgez(args),
      "blt" => Instruction::Blt(args),
      "bgt" => Instruction::Bgt(args),
      "ble" => Instruction::Ble(args),
      "bge" => Instruction::Bge(args),
      "beq" => Instruction::Beq(args),
      "bne" => Instruction::Bne(args),
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
