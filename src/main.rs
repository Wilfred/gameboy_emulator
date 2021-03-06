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
    println!("OFFSET BYTES     INSTR");

    let mut offset = 0;
    while offset < bytes.len() {
        let instr = decode(bytes, offset);
        let byte_count = instr.as_ref().map_or(1, instr_size);

        // Build up a string of bytes for this instr e.g. "FF 00"
        let mut bytes_repr = format!("{:02X}", bytes[offset]);
        for i in 1..byte_count {
            bytes_repr = format!("{} {:02X}", bytes_repr, bytes[offset+i]);
        }
        bytes_repr = format!("{:<9}", bytes_repr);

        // Textual representation of the decode instuction.
        let instr_repr = match instr {
            Some(ref instr) => format!("{:?}", instr),
            None => "???".to_owned()
        };

        println!("  {:04X} {} {}", offset, bytes_repr, instr_repr);
        offset += byte_count;
    }
}

fn print_opcodes_implemented() {
    let mut implemented = 0;
    let mut total = 0;
    for byte1 in 0..256u16 {
        
        if byte1 == 0xCB {
            for byte2 in 0..256u16 {
                let example_sequence = [byte1 as u8, byte2 as u8, 0, 0];

                if decode(&example_sequence, 0).is_some() {
                    implemented += 1;
                }

                total += 1;
            }
        } else {
            let example_sequence = [byte1 as u8, 0, 0];

            if decode(&example_sequence, 0).is_some() {
                implemented += 1;
            }
            
            total += 1;
        }
    }

    println!("Implemented {} of {} instructions ({:.1}%)",
             implemented, total, 100.0 * implemented as f64 / total as f64);
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let path = &args[1];

        if path == "--implemented" {
            print_opcodes_implemented();
            return;
        } else if path == "--demo" {
            println!("DEMO MODE");
            let mut cpu = initial_cpu();
            println!("Initial CPU state: {:?}", cpu);

            let _ = step(&mut cpu, Instruction::Nop);
            println!("Final CPU state:   {:?}", cpu);
            return;
        }
    }

    if args.len() == 3 {
        let command = &args[1];
        let path = &args[2];

        if command == "--dis" {
            // Read a file and print disassembly.
            match read_bytes(path) {
                Ok(bytes) => {
                    print_instrs(&bytes[..]);
                    return;
                }
                Err(_) => {
                    println!("Could not read file: {}", path);
                    std::process::exit(1);
                }
            }
        } else if command == "--run" {
            // Read a file and execute it.
            match read_bytes(path) {
                Ok(bytes) => {
                    match fetch_execute(&bytes[..]) {
                        Ok(_) => {
                            println!("Execution terminatd normally.");
                            return;
                        }
                        Err(msg) => {
                            println!("Failed: {}", msg);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => {
                    println!("Could not read file: {}", path);
                    std::process::exit(1);
                }
            }
        }
    }

    println!("Usage:");
    println!("{} /path/to/rom # disassemble", args[0]);
    println!("{} --implemented # count opcodes we understand", args[0]);
    println!("{} --demo # exercise the emulator", args[0]);
    std::process::exit(1);
}
