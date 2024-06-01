// General purpose emulator for minecraft computers, easy to use (hopefully)
// Originally designed for the redstone computer iris from Mod Punch Tree, however the commands were
// obfuscated and I didn't understand them so I just made my own basic architecture

mod computer;
mod parser;
mod opcodes;
mod types;

#[macro_use]
pub mod error_helper; // Macro for making errors easier to format (it sucks)

use computer::{Computer, ComputerConfig};
use parser::parse_machine_file;
use crate::opcodes::OpcodeDictionary;
use crate::types::OpcodeEntry;

fn main() {
    // Define the configuration for computer (We're basically adding overrides to default)
    let config = ComputerConfig {
        registers: 32,

        // This syntax sets all the other variables from the default defined in computer.rs
        ..Default::default() 
    };

    let file_path = "machine.txt";
    // Make the opcode_entries
    let opcode_entries = match parse_machine_file(file_path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error while parsing opcodes file: {e}");
            return
        },
    };

    // Make the computer (it's a result)
    let mut computer = match Computer::build(config, &opcode_entries){
        Ok(computer) => computer,
        Err(e) => {
            eprintln!("Error while building computer: {e}");
            return
        },
    };

    match fibbonacci(&mut computer, &opcode_entries) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error creating the fibbonacci program: {e}");
            return
        }
    };


    // Definitely need a way to end that's not a panic...
    loop{
        match computer.clock() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Oh no error (I can finally get out of this loop): {e}");
                break;
            },
        };
    };

    println!("Program finished. Exiting...");
}


// Creates an example program for the computer, this being fibbonacci
fn fibbonacci(computer: &mut Computer, opcode_entries: &Vec<OpcodeEntry>) -> Result<(), String> {
    use opcodes::Opcodes::*;

    let opcode_dictionary = OpcodeDictionary::build(opcode_entries)?; // Just call panic Idc
    let instructions_from_enum = [
        LOAD,
        LOAD,
        LOAD,
        ADD,
        ADD,
        SUB,
        JMPZ,
        JMP,
        NOP,
        NOP,
        HALT,
    ];

    let parameters = [
        0, 0, 0, // Load 0 to reg 0
        1, 1, 0, // Load 1 to reg 1
        46368, 2, 0, // Load 46368 to reg 2, because that fibbonacci number and it's alot
        0, 1, 0, // Add 0 and 1 put to 0
        0, 1, 1, // Add 0 and 1 put to 1
        0, 2, 0, // Subtract 2 from 0 and check if 0, then halt
        10, 0, 0, // If last alu op was 0 then jump to line 11 (irom)
        3, 0, 0, // Jump back to line 3 to add all over again
        0, 0, 0, // NOP
        0, 0, 0, // NOP
        0, 0, 0, // HALT! (will give error because Idk how to end gracefully)
    ];

    let mut instructions: Vec<u16> = vec![];
    for opcode in instructions_from_enum.iter() {
        let value = match opcode_dictionary.opcode_to_int(*opcode) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        instructions.push(value as u16);
    }

    computer.irom[..instructions.len()].copy_from_slice(instructions.as_slice());
    computer.rom[..parameters.len()].copy_from_slice(&parameters);

    // Chatgpt explains:
    // rom[..new_values.len()] creates a mutable slice of the rom array from the start to the length of new_values.
    // copy_from_slice(&new_values) copies the elements from new_values into this slice of rom.

    Ok(()) // No error yay
}
