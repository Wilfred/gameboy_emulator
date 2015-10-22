mod instructions;

use instructions::{initial_cpu,addr_e,nop};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut cpu = initial_cpu();
    println!("Initial CPU state: {:?}", cpu);

    addr_e(&mut cpu);
    nop(&mut cpu);
    println!("Final CPU state:   {:?}", cpu);
}
