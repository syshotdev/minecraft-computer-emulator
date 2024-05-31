// Emulates a computer based on some options, instructions and the like.
// Instructions are hard-coded but you can change them if you'd like, it's pretty clean.

// TODO:
// args checking for opcodes
// add rest of commands because I think I'm missing ram ones, and some are not well documented
// I'm doing source -> destination rather than what CPUS actually do, which is dest <- source

use crate::opcodes::{Opcodes, OpcodeDictionary}; // Dictionary for enums to ints
use crate::types::OpcodeEntry;

// Struct telling the compiler how much to allocate for memory
pub struct ComputerConfig {
    // Sizes for components
    pub rom: usize,
    pub ram: usize,
    pub registers: usize,
    pub data_stack: usize,
    pub function_stack: usize,
}

const K: usize = 1024;
// Defaults for sizes of different computer components (like rom and ram)
impl Default for ComputerConfig {
    fn default() -> Self {
        Self {
            rom: 32 * K,
            ram: 8 * K,
            registers: 8,
            data_stack: 256,
            function_stack: 256,
        }
    }
}

pub struct Computer {
    // We're using vectors for arrays because easier to set up (boohoo 10% performance loss)
    rom: Vec<u16>, 
    ram: Vec<u16>,
    registers: Vec<u16>, // For IRIS it was like 30 ish
    data_stack: Vec<u16>,
    function_stack: Vec<u16>, // Return address stack

    program_counter: u16,
    data_stack_pointer: u16,
    function_stack_pointer: u16,

    alu_zero_flag: bool,
    jmped: bool,

    // Dictionary for mapping enums to ints and vice versa
    opcode_dict: OpcodeDictionary,
}

impl Computer {
    // New computer based on config
    pub fn new(config: ComputerConfig, opcode_entries: Vec<OpcodeEntry>) -> Self {
        Self {
            rom: vec![0, config.rom.try_into().unwrap()],
            ram: vec![0, config.ram.try_into().unwrap()],
            registers: vec![0, config.registers.try_into().unwrap()],
            data_stack: vec![0, config.data_stack.try_into().unwrap()],
            function_stack: vec![0, config.function_stack.try_into().unwrap()],

            program_counter: 0,
            data_stack_pointer: 0,
            function_stack_pointer: 0,

            alu_zero_flag: false,
            jmped: false,

            // Others
            opcode_dict: OpcodeDictionary::new(opcode_entries),
        }
    }

    // Simulates one clock cycle for the computer
    // NOT IMPLEMENTED YET
    /*fn clock(&self) {
        const bytes_for_instruction = 4;

        // Fetch instruction from rom and execute it.
        self.execute_opcode(self.rom[self.program_counter], self.rom[self.program_counter+1..self.program_counter+3]);

        // If we jump and then add to program_counter, bad things happen.
        if !self.jmped {
            self.program_counter += bytes_for_instruction; 
        }
    }*/

    // Takes an opcode and args and executes it like the CPU would
    pub fn execute_opcode(&mut self, opcode: Opcodes, args: [u16; 3]) -> Result<(), String> {
        use Opcodes::*; // Include the opcodes so we don't do Opcodes:: all the time
        match opcode {
            NOP => (),
            ADD => self.math_operation(args, |a, b| a + b),
            SUB => self.math_operation(args, |a, b| a - b),
            MULT => self.math_operation(args, |a, b| a * b),
            DIV => self.math_operation(args, |a, b| a / b),
            MOD => self.math_operation(args, |a, b| a % b),

            OR => self.math_operation(args, |a, b| a | b),
            XOR => self.math_operation(args, |a, b| a ^ b),
            AND => self.math_operation(args, |a, b| a & b),
            NAND => self.math_operation(args, |a, b| !(a & b)),
            SHIFTL => self.math_operation(args, |a, b| a << b),
            SHIFTR => self.math_operation(args, |a, b| a >> b),

            NOT => self.unary_operation(args, |a| !a), 

            LOAD => self.registers[args[1] as usize] = args[0], // Load args[0] into register args[1]

            // Store = store reg into ram,
            // Copy = copy ram to reg
            RSTORE => self.ram[args[1] as usize] = self.registers[args[0] as usize],
            RCOPY => self.registers[args[1] as usize] = self.ram[args[0] as usize],
            ICOPY => self.registers[args[1] as usize] = self.rom[args[0] as usize],

            PRINT => {
                let address: usize = args[0] as usize;
                let length: usize = args[1] as usize;
                let u16s_to_convert = &self.ram[address..address+length];

                // Turn u16s into iterator, turn into char, collect into String.
                let string: String = 
                    char::decode_utf16(u16s_to_convert.iter().cloned())
                    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
                    .collect();

                println!("{}", string);
            },

            JMP => { self.program_counter = args[0]; self.jmped = true; },
            JMPZ => if self.alu_zero_flag {self.program_counter = args[0]; self.jmped = true;},

            // None of these matched? Well error.
            _ => return Err(format!("I didn't implement the opcode {}", opcode.to_string())),
        }

        // No error yay
        Ok(())
    }

    // (Registers) args[0] and args[1] are put as parameters into fn operation, then result into args[2]
    fn math_operation<F>(&mut self, args: [u16; 3], operation: F)
    where
        F: Fn(u16, u16) -> u16,
    {    
        self.apply_operation(args[0], Some(args[1]), args[2], operation);
    }

    // Like math_operation except only one regester parameter and one destination
    fn unary_operation<F>(&mut self, args: [u16; 3], operation: F)
    where
        F: Fn(u16) -> u16,
    {
        // Uses helper function and only gives src1 and dst, and function with one perameter.
        self.apply_operation(args[0], None, args[1], |a, _| operation(a));
    }

    // src1 and src2 are registers and they have operation applied and put into register dst.
    fn apply_operation<F>(&mut self, src1: u16, src2: Option<u16>, dst: u16, operation: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        // Register1 always, Register2 IF provided. If only register1, it's unary operation.
        let register1 = self.registers[src1 as usize];
        let register2 = src2.map_or(0, |r| self.registers[r as usize]);

        let result = operation(register1, register2);
        self.alu_zero_flag = result == 0;

        self.registers[dst as usize] = result; // Set destination reg to result
    }
}

