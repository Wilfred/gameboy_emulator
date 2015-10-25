use std::num::Wrapping;

use self::Instruction::*;
use self::Register8Target::*;

// The Z80 is an 8-bit chip.
type Register8 = Wrapping<u8>;
type Register16 = Wrapping<u16>;

// TODO: this should wrap.
type ProgramCounter = u16;

#[derive(Debug)]
pub struct CPU {
    // Generic registers.
    a: Register8,
    b: Register8,
    c: Register8,
    d: Register8,
    e: Register8,
    h: Register8,
    l: Register8,

    flags: Register8,

    // Program state.
    pc: ProgramCounter,
    sp: Register16,

    // Clock.
    m: Register8,
    t: Register8,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Register8Target {
    A,
    B,
    C,
    D,
    E,
    H,
    L
    // TODO: BC, DE, HL, (HL), SP
}

#[derive(Debug,PartialEq,Eq)]
pub enum Instruction {
    Nop,
    Load(RegisterTarget,u16),
    // Load(RegisterTarget,u16),
    Increment(Register8Target),
}

pub fn initial_cpu() -> CPU {
    CPU {
        a: Wrapping(0), b: Wrapping(0),
        c: Wrapping(0), d: Wrapping(0),
        e: Wrapping(0), h: Wrapping(0), l: Wrapping(0),
        flags: Wrapping(0),
        pc: 0, sp: Wrapping(0),
        m: Wrapping(0), t: Wrapping(0),
    }
}

// Get a mutable reference to targeted register.
fn register8(cpu: &mut CPU, target: Register8Target) -> &mut Register8 {
    match target {
        A => { &mut cpu.a }
        B => { &mut cpu.b }
        C => { &mut cpu.c }
        D => { &mut cpu.d }
        E => { &mut cpu.e }
        H => { &mut cpu.h }
        L => { &mut cpu.l }
    }
}

pub fn decode(bytes: &[u8]) -> Instruction {
    match bytes[0] {
        0x00 => {
            Nop
        }
        0x31 => {
            Load(SP, decode_immediate16(&bytes[1..]))
        }
        _ => unimplemented!()
    }
}

/// Decode bytes as a little-endian 16-bit integer.
fn decode_immediate16(bytes: &[u8]) -> u16 {
    let low_byte = bytes[0] as u16;
    let high_byte = bytes[1] as u16;

    (high_byte << 8) + low_byte
}

pub fn step(cpu: &mut CPU, i: Instruction) {
    cpu.pc += 1;
    cpu.m = Wrapping(1);
    cpu.t = Wrapping(4);

    match i {
        Nop => {}
        Increment(target) => {
            // TODO: flags
            let mut reg = register8(cpu, target);
            *reg = *reg + Wrapping(1);
        }
        Load(SP,amount) => {
            cpu.sp = cpu.sp + Wrapping(amount);
        }
        Load(_, _) => unimplemented!()
    }
}

#[test]
fn decode_nop() {
    let bytes = [0x00];
    assert_eq!(decode(&bytes), Nop);
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

#[test]
fn decode_sp_immediate() {
    let bytes = [0x31, 0xFE, 0xFF];
    let instr = decode(&bytes);
    assert_eq!(instr, Load(SP, 0xFFFE));
}

#[test]
fn load_sp_immediate() {
    let bytes = [0x31, 0xFE, 0xFF];
    let mut cpu = initial_cpu();

    step(&mut cpu, decode(&bytes));
    assert_eq!(cpu.sp, Wrapping(0xFFFE));
}
