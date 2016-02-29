// Add Immediate Unsigned Word
use error::{ExecResult, ExecError, ErrorKind};
use decoded::{IO, Opcode, Decodable, Decoded};
use decoder::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddIU {
    rs: u8,
    rt: u8,
    immediate: u16,
}

impl AddIU {
    pub fn new(bitfield: u32) -> Self {
        AddIU {
            rs: extract_25_21(bitfield),
            rt: extract_20_16(bitfield),
            immediate: extract_15_0(bitfield),
        }
    }
}

impl Decodable for AddIU {
    fn opcode() -> Opcode {
        Opcode::Normal(0b001001)
    }
}

impl Decoded for AddIU {
    fn name(&self) -> &'static str {
        "ADDIU"
    }
    fn inputs(&self) -> Vec<IO> {
        vec![IO::Register(self.rs)]
    }
    fn outputs(&self) -> Option<IO> {
        Some(IO::Register(self.rt))
    }
    fn execute(&self, registers: &[u32]) -> ExecResult<u32> {
        assert!(registers.len() == 1);
        let rs = registers[0];
        let rd = rs.wrapping_add(self.immediate as u32);
        ExecResult::Success(rd)
    }
}

#[test]
fn addiu_decode() {
    assert_eq!(AddIU::new(0b001000_00000_11111_0000000000000000u32),
               AddIU {
                   rs: 0b00000,
                   rt: 0b11111,
                   immediate: 0,
               });
    assert_eq!(AddIU::new(0b001000_11111_00000_1111111111111111u32),
               AddIU {
                   rs: 0b11111,
                   rt: 0b00000,
                   immediate: 0xffff,
               });
}

#[test]
fn addiu_exec() {
    let s = AddIU {
        rs: 0,
        rt: 1,
        immediate: 52,
    };
    assert_eq!(s.execute(&[10]).unwrap(), 62);
}

#[test]
fn addiu_exec_overflow() {
    let s = AddIU {
        rs: 0,
        rt: 1,
        immediate: 2,
    };
    match s.execute(&[0xff_ff_ff_ff]) {
        ExecResult::Success(v) => assert_eq!(v, 1),
        ExecResult::Exception(_) => panic!("expected wrap, got exception"),
        _ => panic!("expected wrap, got error"),
    }
}
