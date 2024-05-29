// This is an emulator for the IRIS computer built by Mod Punchtree, however their opcodes are
// cryptic and make absolutely no sense. And I don't know if this is how it works under the hood,
// but oh well ¯\_(ツ)_/¯

// TODO: Put these opcodes into another file and import them, and assign numbers.
// Also args checking would be nice
// Also add rest of commands because I think I'm missing ram ones, and some are not well documented
// Also I'm doing source -> destination rather than what CPUS actually do, which is dest <- source

use strum_macros::{EnumString, Display};

// Just so you know, lowest bit on the right
#[derive(EnumString, Display)]
enum Opcodes {
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
    DEBUG, // First operand pointer to memory, second length, interpreted as chars and printed

    // Halt!
    HALT,
}

const K: usize = 1024;

#[derive(Debug)]
struct Computer {
    rom: [u16; 32 * K], // Rom 64kb
    ram: [u16; 4 * K], // Ram, 8kb
    registers: [u16; 8], // Actual registers is 26-29, Idk
    data_stack: [u16; 128], // Arbitrary number for stack
    function_stack: [u16; 128], // Return address stack

    program_counter: u16,
    data_stack_pointer: u16,
    function_stack_pointer: u16,

    alu_zero_flag: bool,
}

impl Computer {
    fn new() -> Self {
        Self {
            rom: [0; 32 * K],
            ram: [0; 4 * K],
            registers: [0; 8],
            data_stack: [0; 128],
            function_stack: [0; 128],

            program_counter: 0,
            data_stack_pointer: 0,
            function_stack_pointer: 0,

            alu_zero_flag: false,
        }
    }
    fn execute_opcode(&mut self, opcode: Opcodes, args: [u16; 3]) -> Result<(), String> {
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

            NOT => self.unary_operation(args, |a| !a), // I hate how this uses another function 

            LOAD => self.registers[args[1] as usize] = args[0], // Load args[0] into register args[1]

            // Store = store reg into ram,
            // Copy = copy ram to reg
            RSTORE => self.ram[args[1] as usize] = self.registers[args[0] as usize],
            RCOPY => self.registers[args[1] as usize] = self.ram[args[0] as usize],
            ICOPY => self.registers[args[1] as usize] = self.rom[args[0] as usize],

            DUBUG => {
                let address: usize = args[0] as usize;
                let length: usize = args[1] as usize;
                let u16s_to_convert = &self.ram[address..address+length];

                // Turn u16s into iterator, turn into char, collect into String.
                let string: String = 
                    char::decode_utf16(u16s_to_convert.iter().cloned())
                    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
                    .collect();

                println!(string)
            },

            JMP => self.program_counter = args[0],
            JMPZ => if self.alu_zero_flag {self.program_counter = args[0]},

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

fn main() {
    let mut computer = Computer::new();

    computer.registers[0] = 10;
    computer.registers[1] = 20;

    match computer.execute_opcode(Opcodes::ADD, [0, 1, 2]) {
        Ok(()) => (),
        Err(e) => panic!("Error: {}", e),
    }

    println!("Registers: {:?}", &computer.registers);
}

