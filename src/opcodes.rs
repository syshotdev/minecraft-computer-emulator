// Contrary to the name, this isn't only for opcodes. This also handles conversions between the
// enum opcodes and the binary opcodes, enum to binary (aka OpcodeEntry) defined in parser.rs


use std::collections::HashMap;
use strum_macros::{EnumString, Display};
use crate::types::{OpcodeEntry};
use std::str::FromStr; // THIS IS IMPORTANT DON"T REMOVE!

#[derive(EnumString, Display, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Opcodes {
    NOP = 0,
    ADD,
    SUB,
    MULT,
    DIV,
    MOD,

    // Binary ones
    OR,
    XOR,
    NOR,
    NAND,
    AND,
    NOT,
    SHIFTL, // There's BSR and RSH. I think they're different but I don't know how
    SHIFTR,

    // Register things
    LOAD, // Loads first operand (Literal) to second operand (register)

    // Stack
    HPUSH, // I think this means call stack push/pull Edit: It's something different but still
    // confusing
    HPOP,
    CALLR, // If I had to guess, means call register and go to that address
    RETURN,

    // Ram
    RSTORE, // Copy register to ram address
    RCOPY, // Copy ram address to register

    // Rom
    ICOPY, // Copy immediate rom address to register

    
    // Ifs and whatnot
    JMP,
    JMPZ, // JuMP if Zero

    // Debug
    PRINT, // First operand pointer to memory, second length, interpreted as chars and printed

    // Halt!
    HALT,
}

pub struct OpcodeDictionary {
    to_int: HashMap<Opcodes, usize>,
    from_int: HashMap<usize, Opcodes>,
}

impl OpcodeDictionary {
    // Takes the opcodes from the parser and maps the enum to the number, so that binaries for the
    // computer can be read and executed.
    pub fn new(entries: Vec<OpcodeEntry>) -> Self {
        let mut to_int = HashMap::new();
        let mut from_int = HashMap::new();

        for entry in entries {
            // These lines of code turn the strings from OpcodeEntry into actual enums
            // And put enums into the hashmaps
            if let Ok(opcode) = Opcodes::from_str(&entry.opcode) {
                let value = entry.number;
                to_int.insert(opcode, value);
                from_int.insert(value, opcode);
            } else {
                panic!("Unknown opcode {}, please define one in the opcodes.rs and implement it in computer.rs",
                entry.opcode)
            }
        };

        OpcodeDictionary { to_int, from_int }
    }

    pub fn opcode_to_int(&self, opcode: Opcodes) -> Option<usize> {
        self.to_int.get(&opcode).cloned()
    }

    pub fn int_to_opcode(&self, value: usize) -> Option<Opcodes> {
        self.from_int.get(&value).cloned()
    }
}
