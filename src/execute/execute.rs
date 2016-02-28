use std::num::Wrapping;

use decode::*;
use processor::Processor;
use super::error::{ExecResult, ExecError, ErrorKind};

#[cfg(test)]
use quickcheck_extra::masked_quickcheck;
#[cfg(test)]
const EXEC_NUM_CHECKS: usize = 100;



fn add(p: &mut Processor, i: Special) -> ExecResult<()> {
    if let Some(re) = p.reg[i.rs as usize].checked_add(p.reg[i.rt as usize]) {
        p.reg[i.rd as usize] = re;
        Ok(())
    } else {
        Err(ExecError::new(ErrorKind::Overflow, "ADD overflow"))
    }
}

fn add_immediate(p: &mut Processor, i: Immediate) -> ExecResult<()> {
    if let Some(re) = p.reg[i.rs as usize].checked_add(i.immediate as u32) {
        p.reg[i.rt as usize] = re;
        Ok(())
    } else {
        Err(ExecError::new(ErrorKind::Overflow, "ADDI overflow"))
    }
}

fn add_immediate_unsigned(p: &mut Processor, i: Immediate) -> ExecResult<()> {
    let Wrapping(re) = Wrapping(p.reg[i.rs as usize]) + Wrapping(i.immediate as u32);
    p.reg[i.rt as usize] = re;
    Ok(())
}

fn add_unsigned(p: &mut Processor, i: Special) -> ExecResult<()> {
    let Wrapping(re) = Wrapping(p.reg[i.rs as usize]) + Wrapping(p.reg[i.rt as usize]);
    p.reg[i.rd as usize] = re;
    Ok(())
}

// ADD
#[test]
fn exec_add() {
    fn prop(a: u32, b: u32) -> bool {
        let mut p = Processor::new();
        p.reg[0] = a;
        p.reg[1] = b;
        add(&mut p,
            Special {
                opcode: 0,
                rs: 0,
                rt: 1,
                rd: 2,
            })
            .unwrap();
        p.reg[2] == (a + b)
    }
    masked_quickcheck(0x7fffffff).tests(EXEC_NUM_CHECKS).quickcheck(prop as fn(u32, u32) -> bool);
}

#[test]
#[should_panic]
fn exec_add_check_overflow() {
    let mut p = Processor::new();
    p.reg[0] = 0x80_00_00_00;
    p.reg[1] = 0x80_00_00_00;
    add(&mut p,
        Special {
            opcode: 0,
            rs: 0,
            rt: 1,
            rd: 2,
        })
        .unwrap();
}

// ADDI
#[test]
fn exec_add_immediate() {
    fn prop(a: u32, b: u16) -> bool {
        let mut p = Processor::new();
        p.reg[0] = a;
        add_immediate(&mut p,
                      Immediate {
                          opcode: 0,
                          rs: 0,
                          rt: 1,
                          immediate: b,
                      })
            .unwrap();
        p.reg[1] == (a + (b as u32))
    }
    masked_quickcheck(0xff_ff_7f_ff)
        .tests(EXEC_NUM_CHECKS)
        .quickcheck(prop as fn(u32, u16) -> bool);
}

#[test]
#[should_panic]
fn exec_add_immediate_check_overflow() {
    let mut p = Processor::new();
    p.reg[0] = 0xff_ff_ff_ff;
    add_immediate(&mut p,
                  Immediate {
                      opcode: 0,
                      rs: 0,
                      rt: 1,
                      immediate: 1,
                  })
        .unwrap();
}


// ADDIU
#[test]
fn exec_add_immediate_unsigned() {
    fn prop(a: u32, b: u16) -> bool {
        let mut p = Processor::new();
        p.reg[0] = a;
        add_immediate_unsigned(&mut p,
                               Immediate {
                                   opcode: 0,
                                   rs: 0,
                                   rt: 1,
                                   immediate: b,
                               })
            .unwrap();
        p.reg[1] == (Wrapping(a) + Wrapping(b as u32)).0
    }
    masked_quickcheck(0xff_ff_ff_ff)
        .tests(EXEC_NUM_CHECKS)
        .quickcheck(prop as fn(u32, u16) -> bool);
}

#[test]
fn exec_add_immediate_unsigned_okay_overflow() {
    let mut p = Processor::new();
    p.reg[0] = 0xff_ff_ff_ff;
    add_immediate_unsigned(&mut p,
                           Immediate {
                               opcode: 0,
                               rs: 0,
                               rt: 1,
                               immediate: 2,
                           })
        .unwrap();
    assert_eq!(p.reg[1], 1);
}


// ADDU
#[test]
fn exec_add_unsigned() {
    fn prop(a: u32, b: u32) -> bool {
        let mut p = Processor::new();
        p.reg[0] = a;
        p.reg[1] = b;
        add_unsigned(&mut p,
                     Special {
                         opcode: 0,
                         rs: 0,
                         rt: 1,
                         rd: 2,
                     })
            .unwrap();
        p.reg[2] == (Wrapping(a) + Wrapping(b)).0
    }
    masked_quickcheck(0xff_ff_ff_ff)
        .tests(EXEC_NUM_CHECKS)
        .quickcheck(prop as fn(u32, u32) -> bool);
}

#[test]
fn exec_add_unsigned_okay_overflow() {
    let mut p = Processor::new();
    p.reg[0] = 0xff_ff_ff_ff;
    p.reg[1] = 0xff_ff_ff_ff;
    add_unsigned(&mut p,
                 Special {
                     opcode: 0,
                     rs: 0,
                     rt: 1,
                     rd: 2,
                 })
        .unwrap();
    assert_eq!(p.reg[2], 0xff_ff_ff_fe);
}
