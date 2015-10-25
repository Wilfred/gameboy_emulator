use std::num::Wrapping;

use self::Instruction::*;
use self::RegisterTarget::*;

// The Z80 is an 8-bit chip.
type Register = Wrapping<u8>;
// TODO: these should wrap.
type ProgramCounter = u16;
type StackPointer = u16;

#[derive(Debug)]
pub struct CPU {
    // Generic registers.
    a: Register,
    b: Register,
    c: Register,
    d: Register,
    e: Register,
    h: Register,
    l: Register,

    flags: Register,

    // Program state.
    pc: ProgramCounter,
    sp: StackPointer,

    // Clock.
    m: Register,
    t: Register,
}

#[derive(Debug,PartialEq,Eq)]
pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    // TODO: BC, DE, HL, (HL)
    SP
}

#[derive(Debug,PartialEq,Eq)]
pub enum Instruction {
    Nop,
    Increment(RegisterTarget),
}

pub fn initial_cpu() -> CPU {
    CPU {
        a: Wrapping(0), b: Wrapping(0),
        c: Wrapping(0), d: Wrapping(0),
        e: Wrapping(0), h: Wrapping(0), l: Wrapping(0),
        flags: Wrapping(0),
        pc: 0, sp: 0, m: Wrapping(0), t: Wrapping(0),
    }
}

pub fn step(cpu: &mut CPU, i: Instruction) {
    cpu.pc += 1;
    cpu.m = Wrapping(1);
    cpu.t = Wrapping(4);

    match i {
        Nop => {}
        Increment(target) => {
            match target {
                A => {
                    // TODO: flags
                    cpu.a = cpu.a + Wrapping(1);
                }
                _ => unimplemented!()
            }
        }
    }
}

#[test]
fn step_nop() {
    let mut cpu = initial_cpu();

    step(&mut cpu, Nop);
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.m, Wrapping(1));
    assert_eq!(cpu.t, Wrapping(4));
}

#[test]
fn step_inc() {
    let mut cpu = initial_cpu();

    step(&mut cpu, Increment(A));
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.m, Wrapping(1));
    assert_eq!(cpu.t, Wrapping(4));

    assert_eq!(cpu.a, Wrapping(1));
}

#[test]
fn step_inc_wraps() {
    let mut cpu = initial_cpu();
    cpu.a = Wrapping(255);

    step(&mut cpu, Increment(A));
    assert_eq!(cpu.a, Wrapping(0));
}
