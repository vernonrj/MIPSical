use std::collections::HashMap;

use error::ExecResult;
use decoded::Opcode;
use decoder::{Register, Instruction};

pub fn register(m: &mut HashMap<Opcode, Instruction>) {
    fn my_and(inputs: &Vec<u32>) -> ExecResult<u32> {
        let (rs, rt) = (inputs[0], inputs[1]);
        ExecResult::Success(rs & rt)
    }
    m.insert(Opcode::Special(0b100100),
             Instruction {
                 name: "AND",
                 inputs: vec![Register::RS, Register::RT],
                 outputs: Some(Register::RT),
                 execute: Box::new(my_and),
                 ..Default::default()
             });
    m.insert(Opcode::Normal(0b001100),
             Instruction {
                 name: "ANDI",
                 inputs: vec![Register::RS, Register::Immediate],
                 outputs: Some(Register::RT),
                 execute: Box::new(my_and),
                 ..Default::default()
             });
}

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn and_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000000_11111_00000_11111_00000_100100u32))
                 .unwrap();
        assert_eq!(i.name, "AND");
    }
    #[test]
    fn andi_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b001100_11111_00000_1111111111111111u32))
                 .unwrap();
        assert_eq!(i.name, "ANDI");
    }
}
