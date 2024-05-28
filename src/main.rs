// This is an emulator for the IRIS computer built by Mod Punchtree, however their opcodes are
// cryptic and make absolutely no sense. And I don't know if this is how it works under the hood,
// but oh well ¯\_(ツ)_/¯

// TODO: Put these opcodes into another file and import them, and assign numbers.
// Also args checking would be nice
// Also add rest of commands because I think I'm missing ram ones, and some are not well documented
// Also the machine uses 16 bit data not 8 bit (but who cares)

// Just so you know, lowest bit on the right
enum Opcodes {
    NOP = 0,
    ADD,
    SUB,
    MULT,
    DIV,
    MOD,

    // Binary ones
    NAND,
    AND,
    OR,
    NOT,
    XOR,
    SHIFTL, // There's BSR and RSH. I think they're different but I don't know how
    SHIFTR,

    // Register things
    LOAD, // Loads first operand to second operand register
    LOAD_IMM_ROM, // Loads whatever the current data from rom is into the register you specify



    // Stack
    HPUSH, // I think this means call stack push/pull
    HPOP,
    CALLR, // If I had to guess, means call register and go to that address
    RETURN,

    
    // Ifs and whatnot
    JMP,
    JMPZ, // JuMP if Zero

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
        }
    }
    fn execute_opcode(&mut self, opcode: Opcodes, args: [u16; 3]) -> Result<(), String> {
        use Opcodes::*;
        match opcode {
            Opcodes::NOP => break,
            Opcodes::ADD => self.logic_operation(args, |a, b| a + b),
            Opcodes::SUB => self.logic_operation(args, |a, b| a - b),
            Opcodes::MULT => self.logic_operation(args, |a, b| a * b),
            Opcodes::DIV => self.logic_operation(args, |a, b| a / b),
            Opcodes::MOD => self.logic_operation(args, |a, b| a % b),
            // None of these matched? Well error.
            _ => return Err(format!("I didn't implement the opcode {:#X}", opcode)),
        }
        Ok(())
    }
    // Logic operations like add, sub and whatever that use two 16 bit registers and output one 16
    // bit register. Multiply and divide should give 32 bit, but I don't care so whatever
    fn logic_operation<F>(&mut self, args: [u16; 3], operation: F)
    where
        F: Fn(u16, u16) -> u16,
    {
        let register1 = self.registers[args[0] as usize];
        let register2 = self.registers[args[1] as usize];

        let result = operation(register1, register2);

        self.registers[args[2] as usize] = result;
    }
}

fn main() {
    let mut computer = Computer::new();

    computer.registers[0] = 10;
    computer.registers[1] = 20;

    computer.execute_opcode(Opcodes::ADD, [0, 1, 2]);

    println!("Registers: {:?}", &computer.registers);
}

