// Contrary to the name, this isn't only for opcodes. This also handles conversions between the
// enum opcodes and the binary opcodes, enum to binary defined in parser.rs


use std::collections::HashMap;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Opcode {
    Nop,
    Load,
    Store,
    Add,
    Sub,
    // Add other opcodes here
}

pub struct OpcodeDictionary {
    to_int: HashMap<Opcode, u8>,
    from_int: HashMap<u8, Opcode>,
}

impl OpcodeDictionary {
    pub fn new() -> Self {
        let mut to_int = HashMap::new();
        let mut from_int = HashMap::new();

        // Populate the hashmaps with opcodes and their corresponding values
        to_int.insert(Opcode::Nop, 0x00);
        from_int.insert(0x00, Opcode::Nop);
        to_int.insert(Opcode::Load, 0x01);
        from_int.insert(0x01, Opcode::Load);
        to_int.insert(Opcode::Store, 0x02);
        from_int.insert(0x02, Opcode::Store);
        to_int.insert(Opcode::Add, 0x03);
        from_int.insert(0x03, Opcode::Add);
        to_int.insert(Opcode::Sub, 0x04);
        from_int.insert(0x04, Opcode::Sub);

        // Add other opcodes here

        OpcodeDictionary { to_int, from_int }
    }

    pub fn opcode_to_int(&self, opcode: Opcode) -> Option<u8> {
        self.to_int.get(&opcode).cloned()
    }

    pub fn int_to_opcode(&self, value: u8) -> Option<Opcode> {
        self.from_int.get(&value).cloned()
    }
}

Use the Conversion Logic in Your Computer Struct:

rust

// computer.rs
use crate::opcode::{Opcode, OpcodeDictionary};

pub struct Computer {
    // other fields
    opcode_dict: OpcodeDictionary,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            // initialize other fields
            opcode_dict: OpcodeDictionary::new(),
        }
    }

    pub fn execute_opcode(&self, value: u8) {
        if let Some(opcode) = self.opcode_dict.int_to_opcode(value) {
            match opcode {
                Opcode::Nop => {
                    // Handle NOP
                }
                Opcode::Load => {
                    // Handle Load
                }
                Opcode::Store => {
                    // Handle Store
                }
                Opcode::Add => {
                    // Handle Add
                }
                Opcode::Sub => {
                    // Handle Sub
                }
                // Handle other opcodes here
            }
        } else {
            // Handle invalid opcode
        }
    }
}
