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
    fn inputs(&self) -> Vec<Register>;
    fn outputs(&self) -> Option<Register>;
    fn execute(&self, reg_vals: &[u32]) -> ExecResult<u32>;
}

pub struct Register(pub u8);

