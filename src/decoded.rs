use super::error::ExecResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Opcode {
    Normal(u8),
    Special(u8),
    RegImm(u8),
}

pub trait Decodable {
    fn opcode() -> Opcode;
}

pub trait Decoded {
    fn name(&self) -> &'static str;
    fn inputs(&self) -> Vec<IO>;
    fn outputs(&self) -> Option<IO>;
    fn is_trap_on_overflow(&self) -> bool {
        false
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32>;
}

pub enum IO {
    Register(u8),
    Memory,
}
