use std::collections::HashMap;

use super::decoded::{Decoded, Opcode};
use instruction::shift;

pub struct Decoder {
    instructions: HashMap<Opcode, Box<Fn(u32) -> Box<Decoded> + 'static>>
}

impl Decoder {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        shift::register(&mut m);
        Decoder {
            instructions: m
        }
    }
    pub fn decode(&self, command: Fetched) -> Option<Box<Decoded>> {
        let opcode = {
            let op = extract_31_26(command.0);
            match op {
                0b000000 => Opcode::Special(extract_5_0(command.0)),
                0b000001 => Opcode::RegImm(extract_20_16(command.0)),
                _ => Opcode::Normal(op)
            }
        };
        let i = self.instructions.get(&opcode);
        i.map(|f| f(command.0))
    }
}

pub struct Fetched(pub u32);

pub fn extract_31_26(bitfield: u32) -> u8 {
    ((bitfield >> 26) & 0x3f) as u8
}
pub fn extract_25_21(bitfield: u32) -> u8 {
    ((bitfield >> 21) & 0x1f) as u8
}
pub fn extract_20_16(bitfield: u32) -> u8 {
    ((bitfield >> 16) & 0x1f) as u8
}
pub fn extract_15_11(bitfield: u32) -> u8 {
    ((bitfield >> 11) & 0x1f) as u8
}
pub fn extract_15_0(bitfield: u32) -> u16 {
    (bitfield & 0xffff) as u16
}
pub fn extract_10_6(bitfield: u32) -> u8 {
    ((bitfield >> 6) & 0x1f) as u8
}
pub fn extract_5_0(bitfield: u32) -> u8 {
    (bitfield & 0x3f) as u8
}
pub fn extract_25_0(bitfield: u32) -> u32 {
    bitfield & 0x03ffffff
}

#[test]
fn test_extract_31_26() {
    assert_eq!(extract_31_26(0b111111_11111_11111_11111_11111_111111),
               0b111111);
    assert_eq!(extract_31_26(0b111111_00000_00000_00000_00000_000000),
               0b111111);
    assert_eq!(extract_31_26(0b000000_11111_11111_11111_11111_111111),
               0b000000);
}

#[test]
fn test_extract_25_21() {
    assert_eq!(extract_25_21(0b111111_11111_11111_11111_11111_111111),
               0b11111);
    assert_eq!(extract_25_21(0b000000_11111_00000_00000_00000_000000),
               0b11111);
    assert_eq!(extract_25_21(0b111111_00000_11111_11111_11111_111111),
               0b00000);
}

#[test]
fn test_extract_20_16() {
    assert_eq!(extract_20_16(0b111111_11111_11111_11111_11111_111111),
               0b11111);
    assert_eq!(extract_20_16(0b000000_00000_11111_00000_00000_000000),
               0b11111);
    assert_eq!(extract_20_16(0b111111_11111_00000_11111_11111_111111),
               0b00000);
}

#[test]
fn test_extract_15_11() {
    assert_eq!(extract_15_11(0b111111_11111_11111_11111_11111_111111),
               0b11111);
    assert_eq!(extract_15_11(0b000000_00000_00000_11111_00000_000000),
               0b11111);
    assert_eq!(extract_15_11(0b111111_11111_11111_00000_11111_111111),
               0b00000);
}

#[test]
fn test_extract_15_0() {
    assert_eq!(extract_15_0(0xff_ff_ff_ff), 0xffff);
    assert_eq!(extract_15_0(0x00_00_ff_ff), 0xffff);
    assert_eq!(extract_15_0(0xff_ff_00_00), 0x0000);
}

#[test]
fn test_extract_10_6() {
    assert_eq!(extract_10_6(0b111111_11111_11111_11111_11111_111111),
               0b11111);
    assert_eq!(extract_10_6(0b000000_00000_00000_00000_11111_000000),
               0b11111);
    assert_eq!(extract_10_6(0b111111_11111_11111_11111_00000_111111),
               0b00000);
}


#[test]
fn test_extract_5_0() {
    assert_eq!(extract_5_0(0xff_ff_ff_ff), 0x3f);
    assert_eq!(extract_5_0(0x00_00_00_3f), 0x3f);
    assert_eq!(extract_5_0(0xff_ff_ff_c0), 0x00);
}

#[test]
fn test_extract_25_0() {
    assert_eq!(extract_25_0(0xff_ff_ff_ff), 0x03_ff_ff_ff);
    assert_eq!(extract_25_0(0x03_ff_ff_ff), 0x03_ff_ff_ff);
    assert_eq!(extract_25_0(0xfc_00_00_00), 0x00_00_00_00);
}

#[test]
fn test_decode() {
    let d = Decoder::new();
    let i = d.decode(Fetched(0b000000_00000_11111_00000_11111_000000u32));
    assert_eq!(i.unwrap().name(), "SLL");
}
