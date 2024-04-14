use crate::lexer::tokens::{Type, Value};
use std::fmt;

use super::{
  DataSection, Instruction, InstructionArgument, Program, Statement, TextSection, Variable,
};

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}\n{}", self.data_section, self.text_section)
  }
}

impl fmt::Display for DataSection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let variables_str = self
      .variables
      .iter()
      .map(|var| var.to_string())
      .collect::<Vec<_>>()
      .join("\t\n");
    write!(
      f,
      ".data\n{}{}",
      variables_str,
      if variables_str.is_empty() { "" } else { "\n" }
    )
  }
}

impl fmt::Display for Variable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.type_ {
      Type::Asciiz => {
        if let Value::String(value) = &self.value {
          write!(f, "{name}: .asciiz {value}", name = self.name)
        } else {
          unreachable!()
        }
      }
      Type::Space => {
        if let Value::Bytes(size) = &self.value {
          write!(f, "{name}: .space {size}", name = self.name)
        } else {
          unreachable!()
        }
      }
    }
  }
}

impl fmt::Display for TextSection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let statements_str = self
      .statements
      .iter()
      .map(|stmt| stmt.to_string())
      .collect::<Vec<_>>()
      .join("\n");
    write!(
      f,
      "\t.text\n\t.global {}\n{}",
      self.entrypoint, statements_str
    )
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Statement::Instruction(i) => write!(f, "\t{}", i),
      Statement::Label(l) => write!(f, "{}:", l),
    }
  }
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Instruction::Li(args) => write!(f, "li {}", write_args(args)),
      Instruction::Add(args) => write!(f, "add {}", write_args(args)),
      Instruction::Mul(args) => write!(f, "mul {}", write_args(args)),
      Instruction::Div(args) => write!(f, "div {}", write_args(args)),
      Instruction::La(args) => write!(f, "la {}", write_args(args)),
      Instruction::Syscall => write!(f, "syscall"),
      Instruction::Move(args) => write!(f, "move {}", write_args(args)),
      Instruction::Jal(args) => write!(f, "jal {}", write_args(args)),
      Instruction::Beq(args) => write!(f, "beq {}", write_args(args)),
      Instruction::Sub(args) => write!(f, "sub {}", write_args(args)),
      Instruction::Jr(args) => write!(f, "jr {}", write_args(args)),
      Instruction::Addi(args) => write!(f, "addi {}", write_args(args)),
      Instruction::Andi(args) => write!(f, "andi {}", write_args(args)),
      Instruction::J(args) => write!(f, "j {}", write_args(args)),
      Instruction::Sw(args) => write!(f, "sw {}, 0({})", args[0], args[1]),
      Instruction::Lw(args) => write!(f, "lw {}", write_args(args)),
      Instruction::Slt(args) => write!(f, "slt {}", write_args(args)),
      Instruction::Beqz(args) => write!(f, "beqz {}", write_args(args)),
      Instruction::Bnez(args) => write!(f, "bnez {}", write_args(args)),
      Instruction::Bltz(args) => write!(f, "bltz {}", write_args(args)),
      Instruction::Bgtz(args) => write!(f, "bgtz {}", write_args(args)),
      Instruction::Blez(args) => write!(f, "blez {}", write_args(args)),
      Instruction::Bgez(args) => write!(f, "bgez {}", write_args(args)),
      Instruction::Blt(args) => write!(f, "blt {}", write_args(args)),
      Instruction::Bgt(args) => write!(f, "bgt {}", write_args(args)),
      Instruction::Ble(args) => write!(f, "ble {}", write_args(args)),
      Instruction::Bge(args) => write!(f, "bge {}", write_args(args)),
      Instruction::Bne(args) => write!(f, "bne {}", write_args(args)),
      Instruction::Sle(args) => write!(f, "sle {}", write_args(args)),
      Instruction::Sgt(args) => write!(f, "sgt {}", write_args(args)),
      Instruction::Sge(args) => write!(f, "sge {}", write_args(args)),
      Instruction::Seq(args) => write!(f, "seq {}", write_args(args)),
      Instruction::Sne(args) => write!(f, "sne {}", write_args(args)),
    }
  }
}

fn write_args(args: &[InstructionArgument]) -> String {
  args
    .iter()
    .map(|arg| arg.to_string())
    .collect::<Vec<_>>()
    .join(", ")
}

impl fmt::Display for InstructionArgument {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      InstructionArgument::Register(r) => write!(f, "{}", r.name),
      InstructionArgument::Immediate(i) => write!(f, "{}", i),
      InstructionArgument::Label(l) => write!(f, "{}", l),
      InstructionArgument::Literal(l) => write!(f, "{}", l),
    }
  }
}
