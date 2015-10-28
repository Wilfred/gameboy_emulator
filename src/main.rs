use std::env;
use std::fs::File;
use std::io::Read;

mod instructions;

use instructions::*;

fn read_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = try!(File::open(path));
    let mut bytes = vec![];
    let _ = file.read_to_end(&mut bytes);
    Ok(bytes)
}

fn print_instrs(bytes: &[u8]) {
    println!("OFFSET BYTE");

    for (offset, byte) in bytes.iter().enumerate() {
        println!("{:X}      0x{:X}", offset, byte);
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let path = &args[1];
        match read_bytes(path) {
            Ok(bytes) => {
                print_instrs(&bytes[..]);
            }
            Err(_) => {
                println!("Could not read file: {}", path);
                std::process::exit(1);
            }
        }
        println!("path {:?}", path);
    } else {
        println!("DEMO MODE");
        let mut cpu = initial_cpu();
        println!("Initial CPU state: {:?}", cpu);

        step(&mut cpu, Instruction::Nop);
        println!("Final CPU state:   {:?}", cpu);
    }
}
