use hack_assembler;
use std::fs;

const ADDRESSABLE_MEM_SIZE: usize = 0x7FFF;

struct Memory32K {
    memory : Vec<u16>,
}

impl Memory32K {
    fn new() -> Memory32K {
        return Memory32K {
            memory: vec![0; ADDRESSABLE_MEM_SIZE],
        };
    }
}

struct CPU {
    /// A register
    a : u16,
    
    /// D register
    d : u16,
    
    /// Program counter
    pc: u16,
}

impl CPU {
    fn new() -> CPU {
        return CPU {
            a: 0,
            d: 0,
            pc: 0
        };
    }
}

struct HackEmulator {
    rom : Memory32K,
    num_instructions: usize,
    ram : Memory32K,
    cpu : CPU,
}

// @note: single cycle fetch execute because of partitioned instruction memory
impl HackEmulator {
    fn new() -> HackEmulator {
        return HackEmulator { 
            rom: Memory32K::new(),
            num_instructions: 0,
            ram: Memory32K::new(),
            cpu: CPU::new()
        }; 
    }
    
    /// helper to obtain the 'm register'
    fn m<'a>(&'a mut self) -> &'a mut u16 {
        return &mut self.ram.memory[self.cpu.a as usize];
    }
    
    /// loads a file written in hack assembly into rom
    fn load_rom_from_file(&mut self, source_path: &str) {
        let source = fs::read_to_string(source_path)
            .expect("Something went wrong reading source file");
            
        let mut assembler_output: String = String::new();
        hack_assembler::assemble(&source, &mut assembler_output);
        
        // we have an intermediate binary because we prepopulate rom 
        let mut binary = Vec::new();
        hack_assembler::generate_binary(assembler_output, &mut binary);
        
        // copy the instructions into rom
        // @todo: do we need to track the loaded instruction count? probably should
        self.num_instructions = binary.len();
        for i in 0..self.num_instructions {
            self.rom.memory[i] = binary[i];
        }
    }
    
    fn run(&mut self, hertz: usize) {
        loop {
            
        }
        
    }
    
    /// decodes and executes instructions for duration_in_cycles. Note: 
    /// a fetch/decode/execute is only 1 clock cycle because the rom is partitioned
    fn execute(&mut self, duration_in_cycles: usize) {
        let instruction = self.rom.memory[self.cpu.pc as usize];
        
        // a-instruction
        if instruction & 0x8000 == 0 {
            self.cpu.a = instruction & 0x7FFF;
        } 
        else {
            
        }
        
    }
    
    fn execute_c_instruction(&mut self, instruction: u16) {
        let comp_raw = instruction & 0xFC0;
        let dest_raw = instruction & 0x38;
        let jump_raw = instruction & 0x7;
        let a_raw = instruction & 0x1000;
        
        let comp = 1; 
        // match comp_raw {
        //     0b101010 {
        // 
        //     }
        // };
        
        if dest_raw & 0b001 != 0 {
            *self.m() = comp;
        }
        if dest_raw & 0b010 != 0 {
            self.cpu.d = comp;
        }
        if dest_raw & 0b100 != 0 {
            self.cpu.a = comp;
        }
    }
}

fn main() {
    let mut emulator = HackEmulator::new();
    emulator.load_rom_from_file("test.asm");
    
    // debug dump instructions in rom
    for i in 0..emulator.num_instructions {
        print!("{}\n", emulator.rom.memory[i]);
    }
}