use std::collections::HashMap;

use decoded::{Opcode, Decodable, Decoded};

mod sll;
mod srl;

pub use self::sll::SLL;
pub use self::srl::SRL;

pub fn register(m: &mut HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>) {
    m.insert(SLL::opcode(), Box::new(|opcode| Box::new(SLL::new(opcode))));
    m.insert(SRL::opcode(), Box::new(|opcode| Box::new(SRL::new(opcode))));
}
