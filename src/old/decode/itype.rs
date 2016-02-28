#[cfg(test)]
use quickcheck_extra::masked_quickcheck;

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Special(Special),
    Immediate(Immediate),
    Branch(Branch),
    RegImm(RegImm),
    Jump(Jump),
    Memory(Memory),
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Special {
    pub opcode: u8,
    pub rs: u8, // FIXME: logical shifts have different format
    pub rt: u8,
    pub rd: u8,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Immediate {
    pub opcode: u8,
    pub rs: u8,
    pub rt: u8,
    pub immediate: u16,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Branch {
    pub opcode: u8,
    pub rs: u8,
    pub rt: u8,
    pub offset: u16,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RegImm {
    pub opcode: u8,
    pub rs: u8,
    pub offset: u16,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Jump {
    pub opcode: u8,
    pub instr_index: u32,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Memory {
    pub opcode: u8,
    pub base: u8,
    pub rt: u8,
    pub offset: u16,
}


impl InstructionType {
    pub fn decode(bitfield: u32) -> Self {
        let opcode = extract_31_26(bitfield);
        if opcode == 0b000000 {
            InstructionType::Special(Special {
                opcode: extract_5_0(bitfield),
                rs: extract_rs(bitfield),
                rt: extract_rt(bitfield),
                rd: extract_rd(bitfield),
            })
        } else if opcode == 0b000001 {
            InstructionType::RegImm(RegImm {
                opcode: extract_20_16(bitfield),
                rs: extract_rs(bitfield),
                offset: extract_offset(bitfield),
            })
        } else if (opcode & 0b100000) > 0 || (opcode & 0b011010) == 0b011010 || (opcode == 0b001111) {
            InstructionType::Memory(Memory {
                opcode: extract_31_26(bitfield),
                base: extract_base(bitfield),
                rt: extract_rt(bitfield),
                offset: extract_offset(bitfield),
            })
        } else if (opcode & 0b001000) > 0 {
            InstructionType::Immediate(Immediate {
                opcode: extract_31_26(bitfield),
                rs: extract_rs(bitfield),
                rt: extract_rt(bitfield),
                immediate: extract_immediate(bitfield),
            })
        } else if (opcode & 0b111100) == 0b010000 {
            panic!("Coprocessor operations not implemented ({:06b})", opcode);
        } else if (opcode & 0b000100) > 0 {
            InstructionType::Branch(Branch {
                opcode: extract_31_26(bitfield),
                rs: extract_rs(bitfield),
                rt: extract_rt(bitfield),
                offset: extract_offset(bitfield),
            })
        } else if (opcode & 0b000010) > 0 {
            InstructionType::Jump(Jump {
                opcode: extract_31_26(bitfield),
                instr_index: extract_25_0(bitfield),
            })
        } else {
            panic!("unknown opcode: {:06b}", opcode);
        }
    }
}

pub fn extract_rs(bitfield: u32) -> u8 {
    extract_25_21(bitfield)
}
pub fn extract_base(bitfield: u32) -> u8 {
    extract_25_21(bitfield)
}
pub fn extract_rt(bitfield: u32) -> u8 {
    extract_20_16(bitfield)
}
pub fn extract_rd(bitfield: u32) -> u8 {
    extract_15_11(bitfield)
}
pub fn extract_immediate(bitfield: u32) -> u16 {
    extract_15_0(bitfield)
}
pub fn extract_offset(bitfield: u32) -> u16 {
    extract_15_0(bitfield)
}


fn extract_31_26(bitfield: u32) -> u8 {
    ((bitfield >> 26) & 0x3f) as u8
}
fn extract_25_21(bitfield: u32) -> u8 {
    ((bitfield >> 21) & 0x1f) as u8
}
fn extract_20_16(bitfield: u32) -> u8 {
    ((bitfield >> 16) & 0x1f) as u8
}
fn extract_15_11(bitfield: u32) -> u8 {
    ((bitfield >> 11) & 0x1f) as u8
}
fn extract_15_0(bitfield: u32) -> u16 {
    (bitfield & 0xffff) as u16
}
fn extract_5_0(bitfield: u32) -> u8 {
    (bitfield & 0x3f) as u8
}
fn extract_25_0(bitfield: u32) -> u32 {
    bitfield & 0x03ffffff
}


impl From<u32> for InstructionType {
    fn from(instruction: u32) -> Self {
        InstructionType::decode(instruction)
    }
}

#[test]
fn test_instruction_type_normal() {
    fn prop(xs: u32) -> bool {
        match InstructionType::decode(xs) {
            InstructionType::Special(Special { .. }) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

#[test]
fn test_instruction_type_immediate() {
    fn prop(xs: u32) -> bool {
        match InstructionType::decode((xs & 0x03ffffff) | (0b001000 << 26)) {
            InstructionType::Immediate(Immediate { .. }) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
