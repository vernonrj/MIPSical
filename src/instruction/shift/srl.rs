// Shift Word Right Logical
use error::ExecResult;
use decoded::{IO, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SRL {
    rt: u8,
    rd: u8,
    sa: u8,
}

impl SRL {
    pub fn new(bitfield: u32) -> Self {
        SRL {
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
            sa: extract_10_6(bitfield),
        }
    }
}

impl Decodable for SRL {
    fn opcode() -> Opcode {
        Opcode::Special(0b000010)
    }
}

impl Decoded for SRL {
    fn name(&self) -> &'static str {
        "SRL"
    }
    fn inputs(&self) -> Vec<IO> {
        vec![IO::Register(self.rt)]
    }
    fn outputs(&self) -> Option<IO> {
        Some(IO::Register(self.rd))
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32> {
        assert!(registers.len() == 1);
        let rt = registers[0];
        let rd = rt >> self.sa;
        ExecResult::Success(rd)
    }
}


#[test]
fn srl_construct() {
    assert_eq!(SRL::new(0b000000_00000_11111_00000_11111_000000),
               SRL {
                   rt: 0b11111,
                   rd: 0b00000,
                   sa: 0b11111,
               });
    assert_eq!(SRL::new(0b000000_00000_00000_11111_00000_000000),
               SRL {
                   rt: 0b00000,
                   rd: 0b11111,
                   sa: 0b00000,
               });
}

#[test]
fn srl_exec() {
    let s = SRL {
        rt: 0,
        rd: 1,
        sa: 2,
    };
    assert_eq!(s.execute(&[10]).unwrap(), 2);
}
