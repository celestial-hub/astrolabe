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
    write!(f, ".data\n{}", variables_str)
  }
}

impl fmt::Display for Variable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.type_ {
      Type::Asciiz => {
        match &self.value {
          Value::String(s) => write!(f, "{}: .asciiz \"{}\"", self.name, s),
          // Handle other value types if necessary
        }
      } // Handle other types if necessary
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
      "\t.text\n\t.globl {}\n{}:\n{}",
      self.entrypoint, self.entrypoint, statements_str
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
      Instruction::Li(args) => write!(
        f,
        "li {}",
        args
          .iter()
          .map(|arg| arg.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Instruction::Add(args) => write!(
        f,
        "add {}",
        args
          .iter()
          .map(|arg| arg.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Instruction::La(_) => todo!(),
      Instruction::Syscall => todo!(),
      Instruction::Move(_) => todo!(),
      Instruction::Jal(_) => todo!(),
      Instruction::Beq(_) => todo!(),
      Instruction::Sub(_) => todo!(),
      Instruction::Jr(_) => todo!(),
      Instruction::Addi(_) => todo!(),
      Instruction::Andi(_) => todo!(),
      Instruction::J(_) => todo!(),
    }
  }
}

impl fmt::Display for InstructionArgument {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      InstructionArgument::Register(r) => write!(f, "{}", r.name),
      InstructionArgument::Immediate(i) => write!(f, "{}", i),
      InstructionArgument::Label(l) => write!(f, "{}", l),
      InstructionArgument::Literal(l) => write!(f, "{}", l),
      // Handle other cases if necessary
    }
  }
}
