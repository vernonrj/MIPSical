use std::collections::HashMap;

use decoded::{Opcode, Decodable, Decoded};

mod add;

pub use self::add::ADD;

pub fn register(m: &mut HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>) {
    m.insert(ADD::opcode(), Box::new(|opcode| Box::new(ADD::new(opcode))));
}

