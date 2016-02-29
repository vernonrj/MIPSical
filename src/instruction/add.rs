use std::collections::HashMap;

use error::{ExecResult, ExecError, ErrorKind};
use decoded::Opcode;
use decoder::{Register, Instruction};

pub fn register(m: &mut HashMap<Opcode, Instruction>) {
    fn my_checked_add(inputs: &Vec<u32>) -> ExecResult<u32> {
        let (rs, rt) = (inputs[0], inputs[1]);
        match rs.checked_add(rt) {
            Some(d) => ExecResult::Success(d),
            None => ExecResult::Exception(ExecError::new(ErrorKind::Overflow, "Add: Overflow")),
        }
    };
    fn my_wrapping_add(inputs: &Vec<u32>) -> ExecResult<u32> {
        let (rs, rt) = (inputs[0], inputs[1]);
        ExecResult::Success(rs.wrapping_add(rt))
    }
    m.insert(Opcode::Special(0b100000),
             Instruction {
                 name: "ADD",
                 inputs: vec![Register::RS, Register::RT],
                 outputs: Some(Register::RD),
                 execute: Box::new(my_checked_add),
                 ..Default::default()
             });
    m.insert(Opcode::Normal(0b001000),
             Instruction {
                 name: "ADDI",
                 inputs: vec![Register::RS, Register::Immediate],
                 outputs: Some(Register::RT),
                 execute: Box::new(my_checked_add),
                 ..Default::default()
             });
    m.insert(Opcode::Normal(0b001001),
             Instruction {
                 name: "ADDIU",
                 inputs: vec![Register::RS, Register::Immediate],
                 outputs: Some(Register::RT),
                 execute: Box::new(my_wrapping_add),
                 ..Default::default()
             });
    m.insert(Opcode::Special(0b100001),
             Instruction {
                 name: "ADDU",
                 inputs: vec![Register::RS, Register::RT],
                 outputs: Some(Register::RD),
                 execute: Box::new(my_wrapping_add),
                 ..Default::default()
             });
}

#[cfg(test)]
mod test {
    use decoder::{Fetched, Decoder};
    #[test]
    fn add_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000000_11111_00000_11111_00000_100000u32))
                 .unwrap();
        assert_eq!(i.name, "ADD");
    }
    #[test]
    fn addi_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b001000_00000_11111_0000000000000000u32))
                 .unwrap();
        assert_eq!(i.name, "ADDI");
    }
    #[test]
    fn addiu_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b001001_00000_11111_0000000000000000u32))
                 .unwrap();
        assert_eq!(i.name, "ADDIU");
    }
    #[test]
    fn addu_decode() {
        let d = Decoder::new();
        let i = d.decode(Fetched(0b000000_11111_00000_11111_00000_100001u32))
                 .unwrap();
        assert_eq!(i.name, "ADDU");
    }
}
