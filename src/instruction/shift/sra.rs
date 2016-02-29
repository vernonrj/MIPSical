// Shift Word Right Arithmetic
use decoded::{IO, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SRA {
    rt: u8,
    rd: u8,
    sa: u8,
}

impl SRA {
    pub fn new(bitfield: u32) -> Self {
        SRA {
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
            sa: extract_10_6(bitfield),
        }
    }
}

impl Decodable for SRA {
    fn opcode() -> Opcode {
        Opcode::Special(0b000011)
    }
}

impl Decoded for SRA {
    fn name(&self) -> &'static str {
        "SRA"
    }
    fn inputs(&self) -> Vec<IO> {
        vec![IO::Register(self.rt)]
    }
    fn outputs(&self) -> Option<IO> {
        Some(IO::Register(self.rd))
    }
    fn execute(&self, registers: &[u64]) -> Option<u64> {
        assert!(registers.len() == 1);
        let rt = registers[0];
        let rd = rt >> self.sa;
        Some(rd)
    }
}


#[test]
fn sra_decode() {
    assert_eq!(SRA::new(0b000000_00000_11111_00000_11111_000000),
               SRA {
                   rt: 0b11111,
                   rd: 0b00000,
                   sa: 0b11111,
               });
    assert_eq!(SRA::new(0b000000_00000_00000_11111_00000_000000),
               SRA {
                   rt: 0b00000,
                   rd: 0b11111,
                   sa: 0b00000,
               });
}

#[test]
fn sra_exec() {
    let s = SRA {
        rt: 0,
        rd: 1,
        sa: 2,
    };
    assert_eq!(s.execute(&[50]), Some(12));
}

