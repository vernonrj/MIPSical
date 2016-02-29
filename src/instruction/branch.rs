use std::collections::HashMap;

use error::ExecResult;
use decoded::Opcode;
use decoder::{Register, Instruction};

pub fn register(m: &mut HashMap<Opcode, Instruction>) {
    m.insert(Opcode::Normal(0b000100),
             Instruction {
                 name: "BEQ",
                 inputs: vec![Register::RS, Register::RT],
                 outputs: None,
                 ..Default::default()
             });
}

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn beq_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000100_11111_00000_1111111111111111u32))
                 .unwrap();
        assert_eq!(i.name, "BEQ");
    }
}
