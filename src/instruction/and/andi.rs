// AndI Immediate
use error::ExecResult;
use decoded::{Register, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AndI {
    rs: u8,
    rt: u8,
    immediate: u16,
}

impl AndI {
    pub fn new(bitfield: u32) -> Self {
        AndI {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            immediate: extract_15_0(bitfield),
        }
    }
}

impl Decodable for AndI {
    fn opcode() -> Opcode {
        Opcode::Normal(0b001100)
    }
}

impl Decoded for AndI {
    fn name(&self) -> &'static str {
        "ANDI"
    }
    fn inputs(&self) -> Vec<Register> {
        vec![Register(self.rs)]
    }
    fn outputs(&self) -> Option<Register> {
        Some(Register(self.rt))
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32> {
        assert!(registers.len() == 1);
        let rs = registers[0];
        let rt = rs & (self.immediate as u32);
        ExecResult::Success(rt)
    }
}

#[test]
fn andi_construct() {
    assert_eq!(AndI::new(0b001100_00000_11111_0000000000000000),
               AndI {
                   rs: 0b00000,
                   rt: 0b11111,
                   immediate: 0,
               });
    assert_eq!(AndI::new(0b001100_11111_00000_1111111111111111),
               AndI {
                   rs: 0b11111,
                   rt: 0b00000,
                   immediate: 0xffff,
               });
}

#[test]
fn andi_exec() {
    let s = AndI {
        rs: 0,
        rt: 1,
        immediate: 0xff,
    };
    assert_eq!(s.execute(&[0xcc]).unwrap(), 0xcc);
}
