mod instructions;

use instructions::*;

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut cpu = initial_cpu();
    println!("Initial CPU state: {:?}", cpu);

    addr_e(&mut cpu);
    step(&mut cpu, Instruction::Nop);
    println!("Final CPU state:   {:?}", cpu);
}
