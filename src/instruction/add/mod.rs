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

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn add_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b000000_11111_00000_11111_00000_100000u32))
                    .unwrap();
        assert_eq!(i.name(), "ADD");
    }
    #[test]
    fn addi_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b001000_00000_11111_0000000000000000u32))
                    .unwrap();
        assert_eq!(i.name(), "ADDI");
    }
    #[test]
    fn addiu_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b001001_00000_11111_0000000000000000u32))
                    .unwrap();
        assert_eq!(i.name(), "ADDIU");
    }
    #[test]
    fn addu_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b000000_11111_00000_11111_00000_100001u32))
                    .unwrap();
        assert_eq!(i.name(), "ADDU");
    }
}
