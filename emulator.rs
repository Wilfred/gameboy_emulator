// The Z80 is an 8-bit chip.
type Register = u8;
type ProgramCounter = u16;
type StackPointer = u16;

#[deriving(Show)]
struct CPU {
        // Generic registers.
        a: Register,
        b: Register,
        c: Register,
        d: Register,
        e: Register,
        h: Register,
        l: Register,
        // Flags register.
        f: Register,

        // Program state.
        pc: ProgramCounter,
        sp: StackPointer,

        // Clock.
        m: Register,
        t: Register,
}

fn ADDr_e (cpu: &mut CPU) {
        //! Add E to A, leaving result in A (ADD A, E)
        cpu.a += cpu.e;
}

fn main() {
        let mut cpu = CPU {
                a: 0, b: 0, c: 0, d: 0, e: 5, h: 0,
                l: 0, f: 0, pc: 0, sp: 0, m: 0, t: 0,
        };
        println!("Initial CPU state: {}", cpu);

        cpu.a += cpu.e;
        // ADDr_e(&cpu);
        println!("Final CPU state:   {}", cpu);
}
