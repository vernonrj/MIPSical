use std::collections::HashMap;

use decoded::{Opcode, Decodable, Decoded};

mod and;
mod andi;

pub use self::and::And;
pub use self::andi::AndI;

pub fn register(m: &mut HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>) {
    m.insert(And::opcode(), Box::new(|op| Box::new(And::new(op))));
    m.insert(AndI::opcode(), Box::new(|op| Box::new(AndI::new(op))));
}
