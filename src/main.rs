use std::io::{self, Error, ErrorKind};

struct Registers {
    // General purpose registers.   
    gp_registers: [u8; 16],

    // Used to store memory adresses
    i: u16,
    
    // current executing adress of a program
    program_counter: u16,

    // pointer to the stack location
    stack_pointer: u8,

    // used to store the address that the interpreter shoud return to when finished with a subroutine
    stack: [u16; 16], 
    
    // Registers for delay and sound timers.
    sound_timer: u8,
    delay_timer: u8,
}

struct Chip8 {
    // Memory for our cpu
    memory: [u8; 4096],  

    // Current opcode for our cpu to execute
    current_opcode: u16,
    
    // Registers for our cpu
    registers: Registers,

    // Cpu keyboard
    keyboard: [u8; 16],

    // Cpu display
    display: [[u8; 64]; 32],
}

const FONT_SET: [u8; 80] = 
[
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

fn main() -> io::Result<()> {
    let mut cpu = Chip8::start();

    cpu.load_program("Pong")?;

    Ok(())
}

impl Registers {
    fn init() -> Self {
        Self {
            gp_registers: [0u8; 16],
            i: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0u16; 16],
            sound_timer: 0,
            delay_timer: 0,
        }
    }
}

impl Chip8 {
    fn start() -> Self {
        let mut memory = [0u8; 4096];

        for (i, byte) in FONT_SET.into_iter().enumerate() {
            memory[i] = byte;
        }

        Self {
            memory,
            current_opcode: 0,
            registers: Registers::init(),
            keyboard: [0u8; 16],
            display: [[0u8; 64]; 32],
        }
    }
    
    fn load_program(&mut self, program: &str) -> io::Result<()> {
        let program_data: Vec<u8> = std::fs::read(program)?;

        for (i, byte) in program_data.iter().enumerate() {
            self.memory[i + self.registers.program_counter as usize] = *byte;
        }

        Ok(())
    }

    fn fetch_opcode(&mut self) -> io::Result<()> {
        let opcode_bytes = [self.memory[self.registers.program_counter as usize], self.memory[self.registers.program_counter as usize]];

        self.current_opcode = u16::from_be_bytes(opcode_bytes);

        Ok(())
    }

    fn exec_opcode(&mut self) -> io::Result<()> {
        match self.current_opcode & 0xF000 {
            // CLS
            // Clear the display
            0x00E0 => {
                
            },
            
            //RET
            0x00EE => {

            },

            // JP addr
            // Set program counter to current opcode.
            0x1000 => {
                self.registers.program_counter = self.current_opcode & 0x0FFF;
            },

            // CALL addr
            0x2000 => {
                self.registers.stack[self.registers.stack_pointer as usize] = self.registers.program_counter;    
                self.registers.stack_pointer += 1;
                self.registers.program_counter = self.current_opcode & 0x0FFF;
            },

            // SE Vx
            0x3000 => {

            },
    
            // SNE Vx
            0x4000 => {

            },

            // SE Vx, Vy        
            0x5000 => {
                
            },

            // LD Vx, byte
            // Set register index at x equal to byte
            0x6000 => {
                let register_idx = self.current_opcode & 0x0F00;
                let byte = self.current_opcode & 0x00FF;
                
                self.registers.gp_registers[register_idx as usize] = byte as u8;
            },

            // ADD Vx, byte
            // Add the current register index at x plus the byte
            0x7000 => {
                let register_idx = self.current_opcode & 0x0F00;
                let byte = self.current_opcode & 0x00FF;

                self.registers.gp_registers[register_idx as usize] = self.registers.gp_registers[register_idx as usize] + byte as u8;
            },

            // LD Vx, Vy
            0x8000 => {
                match self.current_opcode & 0x000F {

                    0x0000 => {

                    },

                    0x0001 => {
    
                    },

                    0x0002 => {

                    },


                    0x0003 => {

                    },


                    0x0004 => {

                    },


                    0x0005 => {

                    },


                    0x0006 => {

                    },


                    0x0007 => {

                    },


                    0x000E => {

                    },

                    _ => {

                    },
                };
            },

            // SNE Vx, Vy
            0x9000 => {

            },
    
            // LD i, addr
            // Set i = addr
            0xA000 => {
                self.registers.i = self.current_opcode & 0x0FFF;
            },

            // JP V0, addr
            // Set program counter to addr + V0
            0xB000 => {
                let addr = self.current_opcode & 0x0FFF;
                self.registers.program_counter = addr + self.registers.gp_registers[0] as u16;
            },

            // RND Vx, byte   
            0xC000 => {
                
            }

            // DRW Vx, Vy, nibble
            0xD000 => {

            },

            0xE000 => {
                match self.current_opcode & 0x00FF {
                    _ => {

                    },
                };
            },

            0xF000 => {
                match self.current_opcode & 0x00FF {
                    _ => {

                    },
                };
            },

            _ => {
                return Err(Error::new(ErrorKind::Other, format!("Invald opcode: {}", self.current_opcode)));
            }
        }

        self.registers.program_counter += 2;

        Ok(())
    }
}
