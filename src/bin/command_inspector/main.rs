use std::io::stdin;
use std::env;

use pvm::instruction::{Instruction, InstructionType};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"list".to_owned()) {
        list_all();
        return;
    } else if args.contains(&"makelut".to_owned()) {
        make_lut();
        return;
    }
    let mut buffer = String::new();
    let stdin = stdin();

    println!("You can also invoke this program with 'list' or 'makelut' as parameters\n.")

    println!("Instruction cheatsheet the first two digits are destination and source registers respectivaelly");
    println!("the next digit represents the load size and operation nibbles, and the last digit represents the ALU OP.");
    println!("example: 30f5 will be the or of A and B storing the results in B.\n");

    loop {
        buffer.clear();
        println!("Please input an instruction in HEX without leading '0x' or 'q' to exit, type 'list' to list all valid instructions");
        stdin.read_line(&mut buffer).unwrap();
        let input = buffer.trim_end();

        if input.starts_with("q") {
            println!("exiting");
            break;
        }

        if input == "list" {
            list_all();
            continue;
        }

        let instruction = match u16::from_str_radix(input, 16) {
            Ok(v) => v,
            Err(e) => {
                println!("Error parsing your input {} {}", buffer, e);
                continue;
            }
        };
        dump_instruction(instruction);
    }
}

fn dump_instruction(instruction: u16) {
    let instruction_parsed = Instruction::parse_instruction(instruction);
    let parts = Instruction::split_bits(instruction);
    if instruction_parsed.family() == InstructionType::Invalid {
        println!("Invalid instruction");
    } else if instruction_parsed.family() == InstructionType::Nop {
        println!("NOP instruction");
    }

    println!(
        "family: {:?}; operation: {:?}; load type: {:?}; load size: {:?}; src: {:?}; dest: {:?}",
        instruction_parsed.family(),
        instruction_parsed.operation(),
        instruction_parsed.load_type(),
        instruction_parsed.load_size(),
        instruction_parsed.source(),
        instruction_parsed.destination(),
    );
    println!(
        "bit pattern: {:04b} {:04b} {:02b} {:02b} {:04b}, hex: {:04X}",
        parts.4, parts.3, parts.2, parts.1, parts.0, instruction
    );
}

fn make_lut() {
    for i in 0..=0xFFFF {
        let parsed = Instruction::parse_instruction(i);
        if parsed.family() != InstructionType::Invalid {
            print!("{:04X},", i);
        }
    }
}

fn list_all() {
    let mut ct = 0;
    for i in 0..=0xFFFF {
        let parsed = Instruction::parse_instruction(i);
        if parsed.family() != InstructionType::Invalid {
            dump_instruction(i);
            ct += 1;
        }
    }
    println!("total valid instructions: {}", ct);
}
