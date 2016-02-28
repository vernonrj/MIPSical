// Shift Word Left Logical
use decoded::{IO, Decoded, Executable};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SLL {
    rt: u8,
    rd: u8,
    sa: u8,
}

impl SLL {
    pub fn new(bitfield: u32) -> Self {
        SLL {
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
            sa: extract_10_6(bitfield),
        }
    }
}

impl Decoded for SLL {
    fn name(&self) -> &'static str {
        "SLL"
    }
    fn inputs(&self) -> Vec<IO> {
        vec![IO::Register(self.rt)]
    }
    fn outputs(&self) -> Option<IO> {
        Some(IO::Register(self.rd))
    }
}

impl Executable for SLL {
    fn execute(&self, registers: &[u64]) -> Option<u64> {
        assert!(registers.len() == 1);
        let rt = registers[0];
        let rd = rt << self.sa;
        Some(rd)
    }
}

#[test]
fn sll_decode() {
    assert_eq!(SLL::new(0b000000_00000_11111_00000_11111_000000),
               SLL {
                   rt: 0b11111,
                   rd: 0b00000,
                   sa: 0b11111,
               });
    assert_eq!(SLL::new(0b000000_00000_00000_11111_00000_000000),
               SLL {
                   rt: 0b00000,
                   rd: 0b11111,
                   sa: 0b00000,
               });
}

#[test]
fn sll_exec() {
    let s = SLL {
        rt: 0,
        rd: 1,
        sa: 5,
    };
    assert_eq!(s.execute(&[10]), Some(320));
}
