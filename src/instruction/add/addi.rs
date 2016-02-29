// Add Immediate Word
use error::{ExecResult, ExecError, ErrorKind};
use decoded::{Register, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddI {
    rs: u8,
    rt: u8,
    immediate: u16,
}

impl AddI {
    pub fn new(bitfield: u32) -> Self {
        AddI {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            immediate: extract_15_0(bitfield),
        }
    }
}

impl Decodable for AddI {
    fn opcode() -> Opcode {
        Opcode::Normal(0b001000)
    }
}

impl Decoded for AddI {
    fn name(&self) -> &'static str {
        "ADDI"
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
        let rd = rs.checked_add(self.immediate as u32);
        match rd {
            Some(d) => ExecResult::Success(d),
            None => ExecResult::Exception(ExecError::new(ErrorKind::Overflow, "AddI: Overflow")),
        }
    }
}

#[test]
fn addi_construct() {
    assert_eq!(AddI::new(0b001000_00000_11111_0000000000000000u32),
               AddI {
                   rs: 0b00000,
                   rt: 0b11111,
                   immediate: 0,
               });
    assert_eq!(AddI::new(0b001000_11111_00000_1111111111111111u32),
               AddI {
                   rs: 0b11111,
                   rt: 0b00000,
                   immediate: 0xffff,
               });
}

#[test]
fn addi_exec() {
    let s = AddI {
        rs: 0,
        rt: 1,
        immediate: 52,
    };
    assert_eq!(s.execute(&[10]).unwrap(), 62);
}

#[test]
fn addi_exec_overflow() {
    let s = AddI {
        rs: 0,
        rt: 1,
        immediate: 2,
    };
    match s.execute(&[0xff_ff_ff_ff]) {
        ExecResult::Exception(_) => (),
        ExecResult::Success(v) => panic!("expected overflow, got 0x{:x}", v),
        _ => panic!("expected overflow"),
    }
}
