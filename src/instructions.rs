use std::fmt;
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

#[derive(PartialEq,Eq)]
pub enum Value {
    Register16(Register16),
    Register8(Register8),
    MemoryAddress(Register16),
    MemoryAddressWithOffset(Register8, u16),
    Immediate16(u16),
    Immediate8(u8),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Register16(ref r) => {
                write!(f, "{:?}", r)
            }
            Value::Register8(ref r) => {
                write!(f, "{:?}", r)
            }
            Value::MemoryAddress(ref r) => {
                write!(f, "({:?})", r)
            }
            Value::MemoryAddressWithOffset(ref r, offset) => {
                write!(f, "(${:X}+{:?})", offset, r)
            }
            Value::Immediate16(v) => {
                write!(f, "${:X}", v)
            }
            Value::Immediate8(v) => {
                write!(f, "${:X}", v)
            }
        }
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum Register16 {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Condition {
    NonZero,
}

// The main enum for Z80 instructions. Note that we assume intel
// conventions, so the destination comes *before* the source.
#[derive(Debug,PartialEq,Eq)]
pub enum Instruction {
    Nop,
    Xor(Register8),
    Load(Value, Value),
    LoadDecrement(Value, Value),
    Increment(Value),
    Decrement(Value),
    // First argument is 0-7, annoyingly Rust doesn't have a u3 type.
    Bit(u8, Value),
    JumpRelative(Condition, i8),
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

/// Given a position in a byte array, return the instruction at that
/// point. Based on http://imrannazar.com/Gameboy-Z80-Opcode-Map .
pub fn decode(bytes: &[u8], offset: usize) -> Option<Instruction> {
    match bytes[offset] {
        0x00 => {
            Some(Nop)
        }
        0x03 => {
            Some(Increment(Value::Register16(BC)))
        }
        0x04 => {
            Some(Increment(Value::Register8(B)))
        }
        0x05 => {
            Some(Decrement(Value::Register8(B)))
        }
        0x0B => {
            Some(Decrement(Value::Register16(BC)))
        }
        0x0C => {
            Some(Increment(Value::Register8(C)))
        }
        0x0D => {
            Some(Decrement(Value::Register8(C)))
        }
        0x0E => {
            Some(Load(Value::Register8(C), Value::Immediate8(bytes[offset + 1])))
        }
        0x13 => {
            Some(Increment(Value::Register16(DE)))
        }
        0x14 => {
            Some(Increment(Value::Register8(D)))
        }
        0x15 => {
            Some(Decrement(Value::Register8(D)))
        }
        0x1B => {
            Some(Decrement(Value::Register16(DE)))
        }
        0x1C => {
            Some(Increment(Value::Register8(E)))
        }
        0x1D => {
            Some(Decrement(Value::Register8(E)))
        }
        0x20 => {
            let addr_offset = bytes[offset+1] as i8;
            Some(JumpRelative(Condition::NonZero, addr_offset))
        }
        0x21 => {
            Some(Load(Value::Register16(HL), decode_immediate16(&bytes[offset + 1..])))
        }
        0x23 => {
            Some(Increment(Value::Register16(HL)))
        }
        0x24 => {
            Some(Increment(Value::Register8(H)))
        }
        0x25 => {
            Some(Decrement(Value::Register8(H)))
        }
        0x2B => {
            Some(Decrement(Value::Register16(HL)))
        }
        0x2C => {
            Some(Increment(Value::Register8(L)))
        }
        0x2D => {
            Some(Decrement(Value::Register8(L)))
        }
        0x31 => {
            Some(Load(Value::Register16(SP), decode_immediate16(&bytes[offset + 1..])))
        }
        0x32 => {
            Some(LoadDecrement(
                Value::MemoryAddress(HL),
                Value::Register8(A)))
        }
        0x33 => {
            Some(Increment(Value::Register16(SP)))
        }
        0x34 => {
            Some(Increment(Value::MemoryAddress(HL)))
        }
        0x35 => {
            Some(Decrement(Value::MemoryAddress(HL)))
        }
        0x3B => {
            Some(Decrement(Value::Register16(SP)))
        }
        0x3C => {
            Some(Increment(Value::Register8(A)))
        }
        0x3D => {
            Some(Decrement(Value::Register8(A)))
        }
        0x3E => {
            Some(Load(Value::Register8(A), Value::Immediate8(bytes[offset + 1])))
        }
        0x77 => {
            Some(Load(Value::MemoryAddress(HL), Value::Register8(A)))
        }
        0xA8 => {
            Some(Xor(B))
        }
        0xA9 => {
            Some(Xor(C))
        }
        0xAA => {
            Some(Xor(D))
        }
        0xAB => {
            Some(Xor(E))
        }
        0xAC => {
            Some(Xor(H))
        }
        0xAD => {
            Some(Xor(L))
        }
        0xAF => {
            Some(Xor(A))
        }
        // 0xCB is the prefix for two byte instructions.
        0xCB => {
            match bytes[offset+1] {
                0x7C => {
                    Some(Bit(7, Value::Register8(H)))
                }
                _ => None
            }
        }
        0xE2 => {
            Some(Load(Value::MemoryAddressWithOffset(C, 0xFF00),
                      Value::Register8(A)))
        }
        _ => None
    }
}

/// Given an instruction, return its size in bytes.
pub fn instr_size(instr: &Instruction) -> usize {
    match *instr {
        Nop => 1,
        Xor(_) => 1,
        Increment(_) => 1,
        Decrement(_) => 1,
        Load(_, ref src) => {
            match *src {
                Value::Immediate16(_) => 3,
                Value::Immediate8(_) => 2,
                _ => 1
            }
        },
        LoadDecrement(_, _) => 1,
        Bit(_, _) => 2,
        JumpRelative(_, _) => 2,
    }
}

/// Decode little-endian bytes as a 16-bit integer.
fn decode_immediate16(bytes: &[u8]) -> Value {
    let low_byte = bytes[0] as u16;
    let high_byte = bytes[1] as u16;

    Value::Immediate16((high_byte << 8) + low_byte)
}

pub fn step(cpu: &mut CPU, i: Instruction) {
    cpu.pc += 1;
    cpu.m = Wrapping(1);
    cpu.t = Wrapping(4);

    match i {
        Nop => {}
        Xor(register_name) => {
            let register_value = *register8(cpu, register_name);
            cpu.a = cpu.a ^ register_value;
        }
        Increment(Value::Register8(target)) => {
            // TODO: flags
            let mut reg = register8(cpu, target);
            *reg = *reg + Wrapping(1);
        }
        _ => unimplemented!()
    }
}

#[test]
fn decode_nop() {
    let bytes = [0x00];
    assert_eq!(decode(&bytes, 0).unwrap(), Nop);
}

#[test]
fn decode_offset() {
    let bytes = [0xAF, 0x00];
    let instr = decode(&bytes, 1).unwrap();
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

    step(&mut cpu, Increment(Value::Register8(A)));
    assert_eq!(cpu.pc, 1);
    assert_eq!(cpu.m, Wrapping(1));
    assert_eq!(cpu.t, Wrapping(4));

    assert_eq!(cpu.a, Wrapping(1));
}

#[test]
fn step_inc_wraps() {
    let mut cpu = initial_cpu();
    cpu.a = Wrapping(255);

    step(&mut cpu, Increment(Value::Register8(A)));
    assert_eq!(cpu.a, Wrapping(0));
}

#[test]
fn decode_sp_immediate() {
    let bytes = [0x31, 0xFE, 0xFF];
    let instr = decode(&bytes, 0).unwrap();
    assert_eq!(instr, Load(Value::Register16(SP),
                           Value::Immediate16(0xFFFE)));
}

// Regression test.
#[test]
fn decode_sp_immediate_arbtirary_offset() {
    let bytes = [0xAF, 0x31, 0xFE, 0xFF];
    let instr = decode(&bytes, 1).unwrap();
    assert_eq!(instr, Load(Value::Register16(SP),
                           Value::Immediate16(0xFFFE)));
}

#[test]
fn decode_xor() {
    let bytes = [0xAF];
    let instr = decode(&bytes, 0).unwrap();
    assert_eq!(instr, Xor(A));
}

#[test]
fn step_xor_a() {
    let bytes = [0xAF];
    let mut cpu = initial_cpu();
    cpu.a = Wrapping(5);

    step(&mut cpu, decode(&bytes, 0).unwrap());
    assert_eq!(cpu.a, Wrapping(0));
}

#[test]
fn decode_ld_hl() {
    let bytes = [0x21, 0xFF, 0x9F];
    let instr = decode(&bytes, 0).unwrap();
    assert_eq!(instr, Load(Value::Register16(HL),
                           Value::Immediate16(0x9FFF)));
}

#[test]
fn decode_ldd_hl_a() {
    let bytes = [0x32];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, LoadDecrement(
        Value::MemoryAddress(HL),
        Value::Register8(A)));
}

#[test]
fn decode_bit_7_h() {
    let bytes = [0xCB, 0x7C];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Bit(7, Value::Register8(H)));
}

#[test]
fn decode_jr_nz() {
    let bytes = [0x20, 0xFB];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, JumpRelative(Condition::NonZero, -5));
}

#[test]
fn decode_ld_c() {
    let bytes = [0x0E, 0x11];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Load(Value::Register8(C), Value::Immediate8(0x11)));
}

#[test]
fn decode_ld_a() {
    let bytes = [0x3E, 0x80];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Load(Value::Register8(A), Value::Immediate8(0x80)));
}

#[test]
fn decode_ld_mem_offset() {
    let bytes = [0xE2];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Load(Value::MemoryAddressWithOffset(C, 0xFF00),
                           Value::Register8(A)));
}

#[test]
fn decode_inc_c() {
    let bytes = [0x0C];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Increment(Value::Register8(C)));
}

#[test]
fn decode_dec() {
    let bytes = [0x35];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Decrement(Value::MemoryAddress(HL)));
}

#[test]
fn decode_ld_rel_hl() {
    let bytes = [0x77];
    let instr = decode(&bytes, 0).unwrap();

    assert_eq!(instr, Load(Value::MemoryAddress(HL), Value::Register8(A)));
}

#[test]
fn ld_size() {
    let instr = Load(Value::Register8(A), Value::Immediate8(1));
    assert_eq!(instr_size(&instr), 2);

    let instr = Load(Value::Register16(HL), Value::Immediate16(1));
    assert_eq!(instr_size(&instr), 3);

    let instr = Load(Value::MemoryAddressWithOffset(C, 0xFF00),
                     Value::Register8(A));
    assert_eq!(instr_size(&instr), 1);

    let instr = Load(Value::MemoryAddress(HL), Value::Register8(A));
    assert_eq!(instr_size(&instr), 1);
}
