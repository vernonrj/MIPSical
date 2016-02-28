use super::itype::{InstructionType, Memory, Special, Immediate, Branch, RegImm, Jump};

#[cfg(test)]
use quickcheck_extra::masked_quickcheck;


#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Add(Special),
    AddI(Immediate),
    AddIU(Immediate),
    AddU(Special),
    And(Special),
    AndI(Immediate),
    Beq(Branch),
    BeqL(Branch),
    BgeZ(RegImm),
    BgeZal(RegImm),
    BgeZall(RegImm),
    BgeZl(RegImm),
    BgtZ(Branch),
    BgtZl(Branch),
    BleZ(Branch),
    BleZl(Branch),
    BltZ(RegImm),
    BltZal(RegImm),
    BltZall(RegImm),
    BltZl(RegImm),
    Bne(Branch),
    BneL(Branch),
    Break(Special),
    // CopZ,
    DAdd(Special),
    DAddI(Immediate),
    DAddIU(Immediate),
    DAddU(Special),
    DDiv(Special),
    DDivU(Special),
    Div(Special),
    DivU(Special),
    DMult(Special),
    DMultU(Special),
    DslL(Special),
    DslL32(Special),
    DslLV(Special),
    DsRA(Special),
    DsRA32(Special),
    DsRAV(Special),
    DsRL(Special),
    DsRL32(Special),
    DsRLV(Special),
    DSub(Special),
    DSubU(Special),
    J(Jump),
    JAL(Jump),
    JALR(Special),
    JR(Special),
    LB(Memory),
    LBU(Memory),
    LD(Memory),
    // LDCz(Memory),
    LDL(Memory),
    LDR(Memory),
    LH(Memory),
    LHU(Memory),
    LL(Memory),
    LLD(Memory),
    LUI(Memory),
    LW(Memory),
    // LWCz(Memory),
    LWL(Memory),
    LWR(Memory),
    LWU(Memory),
    MfHi(Special),
    MfLo(Special),
    MovN(Special),
    MovZ(Special),
    MtHi(Special),
    MtLo(Special),
    Mult(Special),
    MultU(Special),
    NOr(Special),
    Or(Special),
    OrI(Immediate),
    // Pref(...),
    SB(Memory),
    SC(Memory),
    SCD(Memory),
    SD(Memory),
    // SDCz(Memory),
    SDL(Memory),
    SDR(Memory),
    SH(Memory),
    SLL(Special),
    SLLV(Special),
    SLt(Special),
    SLtI(Immediate),
    SLtIU(Immediate),
    SRA(Special),
    SRAV(Special),
    SRL(Special),
    SRLV(Special),
    Sub(Special),
    SubU(Special),
    SW(Memory),
    // SWCz(Memory),
    SWL(Memory),
    SWR(Memory),
    SYNC(Special),
    Syscall(Special),
    TEq(Special),
    TEqI(RegImm),
    TGe(Special),
    TGeI(RegImm),
    TGeIU(RegImm),
    TGeU(Special),
    TLt(Special),
    TLtI(RegImm),
    TLtIU(RegImm),
    TLtU(Special),
    TNe(Special),
    TNeI(RegImm),
    XOr(Special),
    XOrI(Immediate),
}

impl From<u32> for Instruction {
    fn from(bitfield: u32) -> Self {
        let typ = InstructionType::decode(bitfield);
        match typ {
            InstructionType::Special(a) => decode_special(a),
            InstructionType::Immediate(i) => decode_immediate(i),
            InstructionType::Branch(b) => decode_branch(b),
            InstructionType::RegImm(b) => decode_regimm(b),
            InstructionType::Jump(j) => decode_jump(j),
            InstructionType::Memory(l) => decode_memory(l),
        }
    }
}

fn decode_special(ityp: Special) -> Instruction {
    match ityp.opcode {
        0b100000 => Instruction::Add(ityp),
        0b100001 => Instruction::AddU(ityp),
        0b100100 => Instruction::And(ityp),
        0b001101 => Instruction::Break(ityp),
        0b101100 => Instruction::DAdd(ityp),
        0b101101 => Instruction::DAddU(ityp),
        0b011110 => Instruction::DDiv(ityp),
        0b011111 => Instruction::DDivU(ityp),
        0b011010 => Instruction::Div(ityp),
        0b011011 => Instruction::DivU(ityp),
        0b011100 => Instruction::DMult(ityp),
        0b011101 => Instruction::DMultU(ityp),
        0b111000 => Instruction::DslL(ityp),
        0b111100 => Instruction::DslL32(ityp),
        0b010100 => Instruction::DslLV(ityp),
        0b111011 => Instruction::DsRA(ityp),
        0b111111 => Instruction::DsRA32(ityp),
        0b010111 => Instruction::DsRAV(ityp),
        0b111010 => Instruction::DsRL(ityp),
        0b111110 => Instruction::DsRL32(ityp),
        0b010110 => Instruction::DsRLV(ityp),
        0b101110 => Instruction::DSub(ityp),
        0b101111 => Instruction::DSubU(ityp),
        0b001001 => Instruction::JALR(ityp),
        0b001000 => Instruction::JR(ityp),
        0b010000 => Instruction::MfHi(ityp),
        0b010010 => Instruction::MfLo(ityp),
        0b001011 => Instruction::MovN(ityp),
        0b001010 => Instruction::MovZ(ityp),
        0b010001 => Instruction::MtHi(ityp),
        0b010011 => Instruction::MtLo(ityp),
        0b011000 => Instruction::Mult(ityp),
        0b011001 => Instruction::MultU(ityp),
        0b100111 => Instruction::NOr(ityp),
        0b100101 => Instruction::Or(ityp),
        0b000000 => Instruction::SLL(ityp),
        0b000100 => Instruction::SLLV(ityp),
        0b101010 => Instruction::SLt(ityp),
        0b000011 => Instruction::SRA(ityp),
        0b000111 => Instruction::SRAV(ityp),
        0b000010 => Instruction::SRL(ityp),
        0b000110 => Instruction::SRLV(ityp),
        0b100010 => Instruction::Sub(ityp),
        0b100011 => Instruction::SubU(ityp),
        0b001111 => Instruction::SYNC(ityp),
        0b001100 => Instruction::Syscall(ityp),
        0b110100 => Instruction::TEq(ityp),
        0b110000 => Instruction::TGe(ityp),
        0b110001 => Instruction::TGeU(ityp),
        0b110010 => Instruction::TLt(ityp),
        0b110011 => Instruction::TLtU(ityp),
        0b110110 => Instruction::TNe(ityp),
        0b100110 => Instruction::XOr(ityp),
        op @ _ => panic!("invalid opcode: {:06b}", op),
    }
}

fn decode_immediate(ityp: Immediate) -> Instruction {
    match ityp.opcode {
        0b001000 => Instruction::AddI(ityp),
        0b001001 => Instruction::AddIU(ityp),
        0b001100 => Instruction::AndI(ityp),
        0b011000 => Instruction::DAddI(ityp),
        0b011001 => Instruction::DAddIU(ityp),
        0b001101 => Instruction::OrI(ityp),
        0b001010 => Instruction::SLtI(ityp),
        0b001011 => Instruction::SLtIU(ityp),
        0b001110 => Instruction::XOrI(ityp),
        op @ _ => panic!("invalid immediate opcode: {:06b}", op),
    }
}

fn decode_branch(ityp: Branch) -> Instruction {
    match ityp.opcode {
        0b000100 => Instruction::Beq(ityp),
        0b010100 => Instruction::BeqL(ityp),
        0b000111 => Instruction::BgtZ(ityp),
        0b010111 => Instruction::BgtZl(ityp),
        0b000110 => Instruction::BleZ(ityp),
        0b010110 => Instruction::BleZl(ityp),
        0b000101 => Instruction::Bne(ityp),
        0b010101 => Instruction::BneL(ityp),
        op @ _ => panic!("invalid branch opcode: {:06b}", op),
    }
}

fn decode_regimm(ityp: RegImm) -> Instruction {
    match ityp.opcode {
        0b00001 => Instruction::BgeZ(ityp),
        0b10001 => Instruction::BgeZal(ityp),
        0b10011 => Instruction::BgeZall(ityp),
        0b00011 => Instruction::BgeZl(ityp),
        0b00000 => Instruction::BltZ(ityp),
        0b10000 => Instruction::BltZal(ityp),
        0b10010 => Instruction::BltZall(ityp),
        0b00010 => Instruction::BltZl(ityp),
        0b01100 => Instruction::TEqI(ityp),
        0b01000 => Instruction::TGeI(ityp),
        0b01001 => Instruction::TGeIU(ityp),
        0b01010 => Instruction::TLtI(ityp),
        0b01011 => Instruction::TLtIU(ityp),
        0b01110 => Instruction::TNeI(ityp),
        op @ _ => panic!("invalid reg imm opcode: {:05b}", op),
    }
}

fn decode_jump(ityp: Jump) -> Instruction {
    match ityp.opcode {
        0b000010 => Instruction::J(ityp),
        0b000011 => Instruction::JAL(ityp),
        op @ _ => panic!("invalid jump opcode: {:06b}", op),
    }
}

fn decode_memory(ityp: Memory) -> Instruction {
    match ityp.opcode {
        0b100000 => Instruction::LB(ityp),
        0b100100 => Instruction::LBU(ityp),
        0b110111 => Instruction::LD(ityp),
        // 0b110101 | 0b110110 => Instruction::LDCz(ityp),
        0b011010 => Instruction::LDL(ityp),
        0b011011 => Instruction::LDR(ityp),
        0b100001 => Instruction::LH(ityp),
        0b100101 => Instruction::LHU(ityp),
        0b110000 => Instruction::LL(ityp),
        0b110100 => Instruction::LLD(ityp),
        0b001111 => Instruction::LUI(ityp),
        0b100011 => Instruction::LW(ityp),
        0b100010 => Instruction::LWL(ityp),
        0b100110 => Instruction::LWR(ityp),
        0b100111 => Instruction::LWU(ityp),
        0b101000 => Instruction::SB(ityp),
        0b111000 => Instruction::SC(ityp),
        0b111100 => Instruction::SCD(ityp),
        0b111111 => Instruction::SD(ityp),
        0b101100 => Instruction::SDL(ityp),
        0b101101 => Instruction::SDR(ityp),
        0b101001 => Instruction::SH(ityp),
        0b101011 => Instruction::SW(ityp),
        0b101010 => Instruction::SWL(ityp),
        0b101110 => Instruction::SWR(ityp),
        op @ _ => panic!("invalid load opcode: {:06b}", op),
    }
}

// SPECIAL
#[test]
fn test_decode_add() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100000;
        match Instruction::from(instruction) {
            Instruction::Add(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_addu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100001;
        match Instruction::from(instruction) {
            Instruction::AddU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_and() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100100;
        match Instruction::from(instruction) {
            Instruction::And(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_break() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001101;
        match Instruction::from(instruction) {
            Instruction::Break(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dadd() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b101100;
        match Instruction::from(instruction) {
            Instruction::DAdd(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_daddu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b101101;
        match Instruction::from(instruction) {
            Instruction::DAddU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ddiv() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011110;
        match Instruction::from(instruction) {
            Instruction::DDiv(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ddivu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011111;
        match Instruction::from(instruction) {
            Instruction::DDivU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_div() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011010;
        match Instruction::from(instruction) {
            Instruction::Div(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_divu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011011;
        match Instruction::from(instruction) {
            Instruction::DivU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dmult() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011100;
        match Instruction::from(instruction) {
            Instruction::DMult(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dmultu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011101;
        match Instruction::from(instruction) {
            Instruction::DMultU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsll() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111000;
        match Instruction::from(instruction) {
            Instruction::DslL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsll32() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111100;
        match Instruction::from(instruction) {
            Instruction::DslL32(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsllv() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010100;
        match Instruction::from(instruction) {
            Instruction::DslLV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsra() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111011;
        match Instruction::from(instruction) {
            Instruction::DsRA(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsra32() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111111;
        match Instruction::from(instruction) {
            Instruction::DsRA32(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsrav() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010111;
        match Instruction::from(instruction) {
            Instruction::DsRAV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsrl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111010;
        match Instruction::from(instruction) {
            Instruction::DsRL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsrl32() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b111110;
        match Instruction::from(instruction) {
            Instruction::DsRL32(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsrlv() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010110;
        match Instruction::from(instruction) {
            Instruction::DsRLV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsub() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b101110;
        match Instruction::from(instruction) {
            Instruction::DSub(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_dsubu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b101111;
        match Instruction::from(instruction) {
            Instruction::DSubU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_jalr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001001;
        match Instruction::from(instruction) {
            Instruction::JALR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_jr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001000;
        match Instruction::from(instruction) {
            Instruction::JR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_mfhi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010000;
        match Instruction::from(instruction) {
            Instruction::MfHi(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_mflo() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010010;
        match Instruction::from(instruction) {
            Instruction::MfLo(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_movn() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001011;
        match Instruction::from(instruction) {
            Instruction::MovN(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_movz() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001010;
        match Instruction::from(instruction) {
            Instruction::MovZ(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_mthi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010001;
        match Instruction::from(instruction) {
            Instruction::MtHi(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_mtlo() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b010011;
        match Instruction::from(instruction) {
            Instruction::MtLo(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_mult() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011000;
        match Instruction::from(instruction) {
            Instruction::Mult(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_multu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b011001;
        match Instruction::from(instruction) {
            Instruction::MultU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_nor() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100111;
        match Instruction::from(instruction) {
            Instruction::NOr(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_or() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100101;
        match Instruction::from(instruction) {
            Instruction::Or(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sll() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000000;
        match Instruction::from(instruction) {
            Instruction::SLL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sllv() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000100;
        match Instruction::from(instruction) {
            Instruction::SLLV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_slt() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b101010;
        match Instruction::from(instruction) {
            Instruction::SLt(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sra() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000011;
        match Instruction::from(instruction) {
            Instruction::SRA(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_srav() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000111;
        match Instruction::from(instruction) {
            Instruction::SRAV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_srl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000010;
        match Instruction::from(instruction) {
            Instruction::SRL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_srlv() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b000110;
        match Instruction::from(instruction) {
            Instruction::SRLV(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sub() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100010;
        match Instruction::from(instruction) {
            Instruction::Sub(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_subu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100011;
        match Instruction::from(instruction) {
            Instruction::SubU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sync() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001111;
        match Instruction::from(instruction) {
            Instruction::SYNC(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_syscall() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b001100;
        match Instruction::from(instruction) {
            Instruction::Syscall(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_teq() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110100;
        match Instruction::from(instruction) {
            Instruction::TEq(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tge() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110000;
        match Instruction::from(instruction) {
            Instruction::TGe(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tgeu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110001;
        match Instruction::from(instruction) {
            Instruction::TGeU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tlt() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110010;
        match Instruction::from(instruction) {
            Instruction::TLt(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tltu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110011;
        match Instruction::from(instruction) {
            Instruction::TLtU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tne() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b110110;
        match Instruction::from(instruction) {
            Instruction::TNe(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_xor() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03fff800) | 0b100110;
        match Instruction::from(instruction) {
            Instruction::XOr(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// IMMEDIATE
#[test]
fn test_decode_addi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001000 << 26);
        match Instruction::from(instruction) {
            Instruction::AddI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_addiu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001001 << 26);
        match Instruction::from(instruction) {
            Instruction::AddIU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_andi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001100 << 26);
        match Instruction::from(instruction) {
            Instruction::AndI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_daddi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b011000 << 26);
        match Instruction::from(instruction) {
            Instruction::DAddI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_daddiu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b011001 << 26);
        match Instruction::from(instruction) {
            Instruction::DAddIU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ori() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001101 << 26);
        match Instruction::from(instruction) {
            Instruction::OrI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_slti() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001010 << 26);
        match Instruction::from(instruction) {
            Instruction::SLtI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sltiu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001011 << 26);
        match Instruction::from(instruction) {
            Instruction::SLtIU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_xori() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001110 << 26);
        match Instruction::from(instruction) {
            Instruction::XOrI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// BRANCH
#[test]
fn test_decode_beq() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000100 << 26);
        match Instruction::from(instruction) {
            Instruction::Beq(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_beql() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b010100 << 26);
        match Instruction::from(instruction) {
            Instruction::BeqL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bgtz() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000111 << 26);
        match Instruction::from(instruction) {
            Instruction::BgtZ(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bgtzl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b010111 << 26);
        match Instruction::from(instruction) {
            Instruction::BgtZl(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_blez() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000110 << 26);
        match Instruction::from(instruction) {
            Instruction::BleZ(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_blezl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b010110 << 26);
        match Instruction::from(instruction) {
            Instruction::BleZl(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bne() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000101 << 26);
        match Instruction::from(instruction) {
            Instruction::Bne(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bnel() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b010101 << 26);
        match Instruction::from(instruction) {
            Instruction::BneL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// REGIMM
#[test]
fn test_decode_bgez() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b00001 << 16);
        match Instruction::from(instruction) {
            Instruction::BgeZ(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bgezal() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b10001 << 16);
        match Instruction::from(instruction) {
            Instruction::BgeZal(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bgezall() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b10011 << 16);
        match Instruction::from(instruction) {
            Instruction::BgeZall(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bgezl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b00011 << 16);
        match Instruction::from(instruction) {
            Instruction::BgeZl(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bltz() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b00000 << 16);
        match Instruction::from(instruction) {
            Instruction::BltZ(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bltzal() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b10000 << 16);
        match Instruction::from(instruction) {
            Instruction::BltZal(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bltzall() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b10010 << 16);
        match Instruction::from(instruction) {
            Instruction::BltZall(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_bltzl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b00010 << 16);
        match Instruction::from(instruction) {
            Instruction::BltZl(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_teqi() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01100 << 16);
        match Instruction::from(instruction) {
            Instruction::TEqI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tgei() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01000 << 16);
        match Instruction::from(instruction) {
            Instruction::TGeI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tgeiu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01001 << 16);
        match Instruction::from(instruction) {
            Instruction::TGeIU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tlti() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01010 << 16);
        match Instruction::from(instruction) {
            Instruction::TLtI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tltiu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01011 << 16);
        match Instruction::from(instruction) {
            Instruction::TLtIU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_tnei() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03e0ffff) | (1 << 26) | (0b01110 << 16);
        match Instruction::from(instruction) {
            Instruction::TNeI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// JUMP
#[test]
fn test_decode_j() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000010 << 26);
        match Instruction::from(instruction) {
            Instruction::J(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_jal() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b000011 << 26);
        match Instruction::from(instruction) {
            Instruction::JAL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// MEMORY
#[test]
fn test_decode_lb() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100000 << 26);
        match Instruction::from(instruction) {
            Instruction::LB(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lbu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100100 << 26);
        match Instruction::from(instruction) {
            Instruction::LBU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ld() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b110111 << 26);
        match Instruction::from(instruction) {
            Instruction::LD(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
// #[test]
// fn test_decode_ldcz() {
//     fn prop(xs: u32) -> bool {
//         let instruction = (xs & 0x03fffffc) | (0b110100 << 26) | 1;
//         match Instruction::from(instruction) {
//             Instruction::LDCz(..) => true,
//             e @ _ => {
//                 println!("expected LDCz, got {:?}", e);
//                 false
//             }
//         }
//     }
//     masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
// }
#[test]
fn test_decode_ldl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b011010 << 26);
        match Instruction::from(instruction) {
            Instruction::LDL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ldr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b011011 << 26);
        match Instruction::from(instruction) {
            Instruction::LDR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lh() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100001 << 26);
        match Instruction::from(instruction) {
            Instruction::LH(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lhu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100101 << 26);
        match Instruction::from(instruction) {
            Instruction::LHU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_ll() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b110000 << 26);
        match Instruction::from(instruction) {
            Instruction::LL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lld() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b110100 << 26);
        match Instruction::from(instruction) {
            Instruction::LLD(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lui() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b001111 << 26);
        match Instruction::from(instruction) {
            Instruction::LUI(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lw() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100011 << 26);
        match Instruction::from(instruction) {
            Instruction::LW(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lwl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100010 << 26);
        match Instruction::from(instruction) {
            Instruction::LWL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lwr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100110 << 26);
        match Instruction::from(instruction) {
            Instruction::LWR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_lwu() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b100111 << 26);
        match Instruction::from(instruction) {
            Instruction::LWU(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sb() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101000 << 26);
        match Instruction::from(instruction) {
            Instruction::SB(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sc() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b111000 << 26);
        match Instruction::from(instruction) {
            Instruction::SC(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_scd() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b111100 << 26);
        match Instruction::from(instruction) {
            Instruction::SCD(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sd() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b111111 << 26);
        match Instruction::from(instruction) {
            Instruction::SD(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sdl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101100 << 26);
        match Instruction::from(instruction) {
            Instruction::SDL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sdr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101101 << 26);
        match Instruction::from(instruction) {
            Instruction::SDR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sh() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101001 << 26);
        match Instruction::from(instruction) {
            Instruction::SH(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_sw() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101011 << 26);
        match Instruction::from(instruction) {
            Instruction::SW(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_swl() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101010 << 26);
        match Instruction::from(instruction) {
            Instruction::SWL(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}
#[test]
fn test_decode_swr() {
    fn prop(xs: u32) -> bool {
        let instruction = (xs & 0x03ffffff) | (0b101110 << 26);
        match Instruction::from(instruction) {
            Instruction::SWR(..) => true,
            _ => false,
        }
    }
    masked_quickcheck(0x03ffffff).tests(1_000).quickcheck(prop as fn(u32) -> bool);
}

// COPROCESSOR
#[test]
#[should_panic]
fn test_decode_copz() {
    Instruction::from(0b010000 << 26);
}
