use std::collections::HashMap;

use decoded::{Opcode, Decodable, Decoded};

mod sll;
mod srl;
mod sra;

pub use self::sll::SLL;
pub use self::srl::SRL;
pub use self::sra::SRA;

pub fn register(m: &mut HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>) {
    m.insert(SLL::opcode(), Box::new(|opcode| Box::new(SLL::new(opcode))));
    m.insert(SRL::opcode(), Box::new(|opcode| Box::new(SRL::new(opcode))));
    m.insert(SRA::opcode(), Box::new(|opcode| Box::new(SRA::new(opcode))));
}

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn sll_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b000000_11111_00000_11111_000000u32))
                    .unwrap();
        assert_eq!(i.name(), "SLL");
    }
    #[test]
    fn srl_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b000000_00000_11111_00000_11111_000010u32))
                    .unwrap();
        assert_eq!(i.name(), "SRL");
    }
    #[test]
    fn sra_decode() {
        let i = Decoder::new()
                    .decode(Fetched(0b00000_00000_11111_00000_11111_000011u32))
                    .unwrap();
        assert_eq!(i.name(), "SRA");
    }
}
