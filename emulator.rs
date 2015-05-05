// The Z80 is an 8-bit chip.
type Register = u8;
type ProgramCounter = u16;
type StackPointer = u16;

#[derive(Debug)]
struct CPU {
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

static ZERO_FLAG: u8 = 0x80;
static OPERATION_FLAG: u8 = 0x40;
static HALF_CARRY_FLAG: u8 = 0x20;
static CARRY_FLAG: u8 = 0x10;


fn addr_e(cpu: &mut CPU) {
    //! Add E to A, leaving result in A (ADD A, E)

    // We use a larger temporary, so we can detect overflow.
    let mut result: u16 = cpu.a as u16 + cpu.e as u16;

    cpu.flags = 0;
    if result == 0 {
        cpu.flags |= ZERO_FLAG;
    }
    if result > 255 {
        cpu.flags |= CARRY_FLAG;
    }
    
    result = result & 255;
    cpu.a = result as Register;

    cpu.m = 1; cpu.t = 4;
}

fn main() {
    let mut cpu = CPU {
        a: 254, b: 0, c: 0, d: 0, e: 2, h: 0,
        l: 0, flags: 0, pc: 0, sp: 0, m: 0, t: 0,
    };
    println!("Initial CPU state: {:?}", cpu);

    addr_e(&mut cpu);
    println!("Final CPU state:   {:?}", cpu);
}
