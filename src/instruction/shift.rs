use std::collections::HashMap;

use error::ExecResult;
use decoded::Opcode;
use decoder::{Register, Instruction};

pub fn register(m: &mut HashMap<Opcode, Instruction>) {
    fn shift_left_logical(inputs: &Vec<u32>) -> ExecResult<u32> {
        assert!(inputs.len() == 2);
        let (rt, sa) = (inputs[0], inputs[1]);
        ExecResult::Success(rt << sa)
    }
    fn shift_right_logical(inputs: &Vec<u32>) -> ExecResult<u32> {
        assert!(inputs.len() == 2);
        let (rt, sa) = (inputs[0], inputs[1]);
        ExecResult::Success(rt >> sa)
    }
    fn shift_right_arithmetic(inputs: &Vec<u32>) -> ExecResult<u32> {
        assert_eq!(inputs.len(), 2);
        let (rt, sa) = (inputs[0] as i32, inputs[1] as i32);
        ExecResult::Success((rt >> sa) as u32)
    }
    m.insert(Opcode::Special(0b000000),
             Instruction {
                 name: "SLL",
                 inputs: vec![Register::RT, Register::SA],
                 outputs: Some(Register::RD),
                 execute: Box::new(shift_left_logical),
                 ..Default::default()
             });
    m.insert(Opcode::Special(0b000010),
             Instruction {
                 name: "SRL",
                 inputs: vec![Register::RT, Register::SA],
                 outputs: Some(Register::RD),
                 execute: Box::new(shift_right_logical),
                 ..Default::default()
             });
    m.insert(Opcode::Special(0b000011),
             Instruction {
                 name: "SRA",
                 inputs: vec![Register::RT, Register::SA],
                 outputs: Some(Register::RD),
                 execute: Box::new(shift_right_arithmetic),
                 ..Default::default()
             });
}

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn sll_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000000_11111_00000_11111_000000u32))
                 .unwrap();
        assert_eq!(i.name, "SLL");
    }
    #[test]
    fn srl_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000000_00000_11111_00000_11111_000010u32))
                 .unwrap();
        assert_eq!(i.name, "SRL");
    }
    #[test]
    fn sra_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b00000_00000_11111_00000_11111_000011u32))
                 .unwrap();
        assert_eq!(i.name, "SRA");
    }
}
