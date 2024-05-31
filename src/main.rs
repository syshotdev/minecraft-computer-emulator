// General purpose emulator for minecraft computers, easy to use (hopefully)
// Originally designed for the computer emulator from Mod Punch Tree, however the commands were
// obfuscated and I didn't understand them so I just made my own basic architecture

mod computer;
mod parser;
mod opcodes;
mod types;

use computer::{Computer, ComputerConfig};
use parser::parse_machine_file;

fn main() {
    // Define the configuration for computer (We're basically adding overrides to default)
    let config = ComputerConfig {
        registers: 32,

        // This syntax sets all the other variables from the default defined in computer.rs
        ..Default::default() 
    };

    let file_path = "machine.txt";
    let opcode_entries = parse_machine_file(file_path);

    let mut computer = Computer::new(config, opcode_entries);
}

