use std::num::Wrapping;

use self::Instruction::*;
use self::Register8::*;
use self::Register16::*;

// TODO: this should wrap.
type ProgramCounter = u16;

#[derive(Debug)]
pub struct CPU {
    // Generic registers.
    a: Wrapping<u8>,
    b: Wrapping<u8>,
    c: Wrapping<u8>,
    d: Wrapping<u8>,
    e: Wrapping<u8>,
    h: Wrapping<u8>,
    l: Wrapping<u8>,

    flags: Wrapping<u8>,

    // Program state.
    pc: ProgramCounter,
    sp: Wrapping<u16>,

    // Clock.
    m: Wrapping<u8>,
    t: Wrapping<u8>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Register16 {
    // TODO: BC, DE, (HL), SP
    HL,
    SP,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Instruction {
    Nop,
    Xor8(Register8),
    Load16(Register16, u16),
    Increment(Register8),
}

pub fn initial_cpu() -> CPU {
    CPU {
        a: Wrapping(0),
        b: Wrapping(0),
        c: Wrapping(0),
        d: Wrapping(0),
        e: Wrapping(0),
        h: Wrapping(0),
        l: Wrapping(0),
        flags: Wrapping(0),
        pc: 0,
        sp: Wrapping(0),
        m: Wrapping(0),
        t: Wrapping(0),
    }
}

// Get a mutable reference to targeted register.
fn register8(cpu: &mut CPU, target: Register8) -> &mut Wrapping<u8> {
    match target {
        A => {
            &mut cpu.a
        }
        B => {
            &mut cpu.b
        }
        C => {
            &mut cpu.c
        }
        D => {
            &mut cpu.d
        }
        E => {
            &mut cpu.e
        }
        H => {
            &mut cpu.h
        }
        L => {
            &mut cpu.l
        }
    }
}

// Given a position in a byte array, return the instruction at that point.
pub fn decode(bytes: &[u8], offset: usize) -> Instruction {
    match bytes[offset] {
        0x00 => {
            Nop
        }
        0x21 => {
            Load16(HL, decode_immediate16(&bytes[1..]))
        }
        0x31 => {
            Load16(SP, decode_immediate16(&bytes[1..]))
        }
        0xAF => {
            Xor8(A)
        }
        _ => unimplemented!(),
    }
}

/// Decode little-endian bytes as a 16-bit integer.
fn decode_immediate16(bytes: &[u8]) -> u16 {
    let low_byte = bytes[0] as u16;
    let high_byte = bytes[1] as u16;

    (high_byte << 8) + low_byte
}

/// Separate immediate into high and low bytes.
fn split_immediate16(i: u16) -> (u8, u8) {
    let low_byte = i & 0x00FF;
    let high_byte = i & 0xFF00;

    (low_byte as u8, (high_byte >> 8) as u8)
}

pub fn step(cpu: &mut CPU, i: Instruction) {
    cpu.pc += 1;
    cpu.m = Wrapping(1);
    cpu.t = Wrapping(4);

    match i {
        Nop => {}
        Xor8(register_name) => {
            let register_value = *register8(cpu, register_name);
            cpu.a = cpu.a ^ register_value;
        }
        Increment(target) => {
            // TODO: flags
            let mut reg = register8(cpu, target);
            *reg = *reg + Wrapping(1);
        }
        Load16(SP, amount) => {
            cpu.sp = cpu.sp + Wrapping(amount);
        }
        Load16(HL, amount) => {
            let (low, high) = split_immediate16(amount);
            cpu.h = Wrapping(high);
            cpu.l = Wrapping(low);
        }
    }
}

#[test]
fn decode_nop() {
    let bytes = [0x00];
    assert_eq!(decode(&bytes, 0), Nop);
}

#[test]
fn decode_offset() {
    let bytes = [0xAF, 0x00];
    let instr = decode(&bytes, 1);
    assert_eq!(instr, Nop);
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
    let instr = decode(&bytes, 0);
    assert_eq!(instr, Load16(SP, 0xFFFE));
}

#[test]
fn load_sp_immediate() {
    let bytes = [0x31, 0xFE, 0xFF];
    let mut cpu = initial_cpu();

    step(&mut cpu, decode(&bytes, 0));
    assert_eq!(cpu.sp, Wrapping(0xFFFE));
}

#[test]
fn decode_xor() {
    let bytes = [0xAF];
    let instr = decode(&bytes, 0);
    assert_eq!(instr, Xor8(A));
}

#[test]
fn step_xor_a() {
    let bytes = [0xAF];
    let mut cpu = initial_cpu();
    cpu.a = Wrapping(5);

    step(&mut cpu, decode(&bytes, 0));
    assert_eq!(cpu.a, Wrapping(0));
}

#[test]
fn decode_ld_hl() {
    let bytes = [0x21, 0xFF, 0x9F];
    let instr = decode(&bytes, 0);
    assert_eq!(instr, Load16(HL, 0x9FFF));
}

#[test]
fn execute_ld_hl() {
    let bytes = [0x21, 0xFF, 0x9F];
    let mut cpu = initial_cpu();

    step(&mut cpu, decode(&bytes, 0));
    assert_eq!(cpu.h, Wrapping(0x9F));
    assert_eq!(cpu.l, Wrapping(0xFF));
}
