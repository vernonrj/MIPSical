// AddU Unsigned Word
use error::{ExecResult, ExecError, ErrorKind};
use decoded::{IO, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddU {
    rs: u8,
    rt: u8,
    rd: u8,
}

impl AddU {
    pub fn new(bitfield: u32) -> Self {
        AddU {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
        }
    }
}

impl Decodable for AddU {
    fn opcode() -> Opcode {
        Opcode::Special(0b100001)
    }
}

impl Decoded for AddU {
    fn name(&self) -> &'static str {
        "ADDU"
    }
    fn inputs(&self) -> Vec<IO> {
        vec![IO::Register(self.rs), IO::Register(self.rt)]
    }
    fn outputs(&self) -> Option<IO> {
        Some(IO::Register(self.rd))
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32> {
        assert!(registers.len() == 2);
        let rs = registers[0];
        let rt = registers[1];
        let rd = rs.wrapping_add(rt);
        ExecResult::Success(rd)
    }
}

#[test]
fn addu_decode() {
    assert_eq!(AddU::new(0b000000_00000_11111_00000_00000_100001),
               AddU {
                   rs: 0b00000,
                   rt: 0b11111,
                   rd: 0b00000,
               });
    assert_eq!(AddU::new(0b000000_11111_00000_11111_00000_000001),
               AddU {
                   rs: 0b11111,
                   rt: 0b00000,
                   rd: 0b11111,
               });
}

#[test]
fn addu_exec() {
    let s = AddU {
        rs: 0,
        rt: 1,
        rd: 2,
    };
    assert_eq!(s.execute(&[10, 11]).unwrap(), 21);
}

#[test]
fn addu_exec_overflow() {
    let s = AddU {
        rs: 0,
        rt: 1,
        rd: 2,
    };
    match s.execute(&[0xff_ff_ff_ff, 2]) {
        ExecResult::Success(v) => assert_eq!(v, 1),
        _ => panic!("expected wrap"),
    }
}
