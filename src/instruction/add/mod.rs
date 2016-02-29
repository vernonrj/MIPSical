use std::collections::HashMap;

use decoded::{Opcode, Decodable, Decoded};

mod add;
mod addi;
mod addiu;
mod addu;

pub use self::add::Add;
pub use self::addi::AddI;
pub use self::addiu::AddIU;
pub use self::addu::AddU;

pub fn register(m: &mut HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>) {
    m.insert(Add::opcode(), Box::new(|op| Box::new(Add::new(op))));
    m.insert(AddI::opcode(), Box::new(|op| Box::new(AddI::new(op))));
    m.insert(AddIU::opcode(), Box::new(|op| Box::new(AddIU::new(op))));
    m.insert(AddU::opcode(), Box::new(|op| Box::new(AddU::new(op))));
}
