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

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn and_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b000000_11111_00000_11111_00000_100100u32))
                    .unwrap();
        assert_eq!(i.name(), "AND");
    }
    #[test]
    fn andi_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b001100_11111_00000_1111111111111111u32))
                    .unwrap();
        assert_eq!(i.name(), "ANDI");
    }
}
