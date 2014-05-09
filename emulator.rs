// The Z80 is an 8-bit chip.
type Register = u8;

#[deriving(Show)]
struct CPU {
        a: Register,
}

fn main() {
        let cpu = CPU { a: 1 };
        println!("CPU: {}", cpu);
}
