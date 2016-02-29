// Add Word
use error::{ExecResult, ExecError, ErrorKind};
use decoded::{Register, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Add {
    rs: u8,
    rt: u8,
    rd: u8,
}

impl Add {
    pub fn new(bitfield: u32) -> Self {
        Add {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            rd: extract_15_11(bitfield),
        }
    }
}

impl Decodable for Add {
    fn opcode() -> Opcode {
        Opcode::Special(0b100000)
    }
}

impl Decoded for Add {
    fn name(&self) -> &'static str {
        "ADD"
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
        let rd = rs.checked_add(rt);
        match rd {
            Some(d) => ExecResult::Success(d),
            None => ExecResult::Exception(ExecError::new(ErrorKind::Overflow, "ADD: Overflow")),
        }
    }
}

#[test]
fn add_construct() {
    assert_eq!(Add::new(0b000000_00000_11111_00000_00000_100000),
               Add {
                   rs: 0b00000,
                   rt: 0b11111,
                   rd: 0b00000,
               });
    assert_eq!(Add::new(0b000000_11111_00000_11111_00000_000000),
               Add {
                   rs: 0b11111,
                   rt: 0b00000,
                   rd: 0b11111,
               });
}

#[test]
fn add_exec() {
    let s = Add {
        rs: 0,
        rt: 1,
        rd: 2,
    };
    assert_eq!(s.execute(&[10, 11]).unwrap(), 21);
}

#[test]
fn add_exec_overflow() {
    let s = Add {
        rs: 0,
        rt: 1,
        rd: 2,
    };
    match s.execute(&[0xff_ff_ff_ff, 1]) {
        ExecResult::Exception(_) => (),
        ExecResult::Success(v) => panic!("expected overflow, got 0x{:x}", v),
        _ => panic!("expected overflow"),
    }
}
