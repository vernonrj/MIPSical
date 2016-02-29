// And
use error::ExecResult;
use decoded::{Register, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct And {
    rs: u8,
    rt: u8,
    rd: u8,
}

impl And {
    pub fn new(bitfield: u32) -> Self {
        And {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
        }
    }
}

impl Decodable for And {
    fn opcode() -> Opcode {
        Opcode::Special(0b100100)
    }
}

impl Decoded for And {
    fn name(&self) -> &'static str {
        "AND"
    }
    fn inputs(&self) -> Vec<Register> {
        vec![Register(self.rs), Register(self.rt)]
    }
    fn outputs(&self) -> Option<Register> {
        Some(Register(self.rd))
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32> {
        assert!(registers.len() == 2);
        let rs = registers[0];
        let rt = registers[1];
        let rd = rs & rt;
        ExecResult::Success(rd)
    }
}

#[test]
fn add_construct() {
    assert_eq!(And::new(0b000000_00000_11111_00000_00000_100100),
               And {
                   rs: 0b00000,
                   rt: 0b11111,
                   rd: 0b00000,
               });
    assert_eq!(And::new(0b000000_11111_00000_11111_00000_100100),
               And {
                   rs: 0b11111,
                   rt: 0b00000,
                   rd: 0b11111,
               });
}

#[test]
fn and_exec() {
    let s = And {
        rs: 0,
        rt: 1,
        rd: 2,
    };
    assert_eq!(s.execute(&[0xff, 0xcc]).unwrap(), 0xcc);
}
