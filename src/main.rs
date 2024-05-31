// General purpose emulator for minecraft computers, easy to use (hopefully)
// Originally designed for the computer emulator from Mod Punch Tree, however the commands were
// obfuscated and I didn't understand them so I just made my own basic architecture

mod computer;
mod parser;

use computer::{Computer, ComputerConfig};
use parser::parse_machine_file;

fn main() {
    let file_path = "machine.txt"; // A file for operations and 
    let entries = parse_machine_file(file_path);

    // Define the configuration for computer (We're basically adding overrides to default)
    let config = ComputerConfig {
        registers: 32,

        // This syntax sets all the other variables from the default defined in computer.rs
        ..Default::default() 
    };

    let mut computer = Computer::new(config);

    println!("Registers: {:?}", &computer.registers);
}

