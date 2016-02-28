mod itype;
mod instruction;

pub use self::itype::{InstructionType, Special, Immediate, Branch, RegImm, Jump, Memory};
pub use self::instruction::Instruction;
