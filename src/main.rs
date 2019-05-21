use hack_assembler;
use std::fs;
use std::thread;

const ADDRESSABLE_MEM_SIZE: usize = 0x7FFF;

struct Memory32K {
    memory : Vec<i16>,
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
    a : i16,
    
    /// D register
    d : i16,
    
    /// Program counter
    pc: i16,
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
    fn m<'a>(&'a mut self) -> &'a mut i16 {
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
        self.num_instructions = binary.len();
        for i in 0..self.num_instructions {
            self.rom.memory[i] = binary[i] as i16;
        }
    }
    
    fn run(&mut self) {
        loop {
            print!("A: {}\nD: {}\nPC: {}\n\n", self.cpu.a, self.cpu.d, self.cpu.pc);
            self.execute(1);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    
    /// decodes and executes instructions for duration_in_cycles. Note: 
    /// a fetch/decode/execute is only 1 clock cycle because the rom is partitioned
    fn execute(&mut self, duration_in_cycles : usize) {
        for _ in 0..duration_in_cycles {
            let instruction = self.rom.memory[self.cpu.pc as usize];
            
            // a-instruction
            let is_jump;
            if instruction as u16 & 0x8000 == 0 {
                self.cpu.a = instruction & 0x7FFF;
                is_jump = false;
            } 
            else {
                is_jump = self.execute_c_instruction(instruction);
            }
            
            if is_jump {
                self.cpu.pc = self.cpu.a;
            }
            else {
                self.cpu.pc += 1;
            }
        }
    }
    
    fn execute_c_instruction(&mut self, instruction: i16) -> bool {
        let comp_raw = (instruction & 0xFC0) >> 6 ;
        let dest_raw = (instruction & 0x38) >> 3;
        let jump_raw = instruction & 0x7;
        let a_raw = instruction & 0x1000;
        
        let comp; 
        match comp_raw {
            0b101010 => comp = 0,
            0b111111 => comp = 1,
            0b111010 => comp = -1,    
            0b001100 => comp = self.cpu.d,
            0b110000 => comp = if a_raw != 0 { *self.m() } else { self.cpu.a },
            0b001101 => comp = !self.cpu.d,
            0b110001 => comp = if a_raw != 0 { !*self.m() } else { !self.cpu.a },
            0b001111 => comp = -self.cpu.d,
            0b110011 => comp = if a_raw != 0 { -*self.m() } else { -self.cpu.a },
            0b011111 => comp = self.cpu.d + 1,
            0b110111 => comp = if a_raw != 0 { *self.m() + 1 } else { self.cpu.a + 1 },
            0b001110 => comp = self.cpu.d - 1,
            0b110010 => comp = if a_raw != 0 { *self.m() - 1 } else { self.cpu.a - 1 },
            0b000010 => comp = if a_raw != 0 { *self.m() + self.cpu.d } else { self.cpu.a + self.cpu.d },
            0b010011 => comp = if a_raw != 0 { self.cpu.d - *self.m() } else { self.cpu.d  - self.cpu.a },
            0b000111 => comp = if a_raw != 0 { *self.m() - self.cpu.d } else { self.cpu.a - self.cpu.d },
            0b000000 => comp = if a_raw != 0 { *self.m() & self.cpu.d } else { self.cpu.a & self.cpu.d },
            0b010101 => comp = if a_raw != 0 { *self.m() | self.cpu.d } else { self.cpu.a | self.cpu.d },
            _ => { panic!("could not decode compute part of c instruction") }
        }
        
        if dest_raw & 0b001 != 0 {
            *self.m() = comp;
        }
        if dest_raw & 0b010 != 0 {
            self.cpu.d = comp;
        }
        if dest_raw & 0b100 != 0 {
            self.cpu.a = comp;
        }
        
        let is_jump;
        match jump_raw {
            0b000 => is_jump = false,
            0b001 => is_jump = comp > 0,
            0b010 => is_jump = comp == 0,
            0b011 => is_jump = comp >= 0,
            0b100 => is_jump = comp < 0,
            0b101 => is_jump = comp != 0,
            0b110 => is_jump = comp <= 0,
            0b111 => is_jump = true,
            _ => { panic!("can't decode jump part of c instruction") }
        }
        
        return is_jump;
    }
}

fn main() {
    let mut emulator = HackEmulator::new();
    emulator.load_rom_from_file("test.asm");
    emulator.run();
    
    // debug dump instructions in rom
    for i in 0..emulator.num_instructions {
        print!("{}\n", emulator.rom.memory[i]);
    }
}