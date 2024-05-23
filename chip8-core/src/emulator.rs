const MEM_SIZE: usize = 0xFFF;
const PC_START: u16 = 0x200;
const STACK_SIZE: usize = 16;
const REG_SIZE: usize = 16;
const FONTSET_SIZE: usize = 80;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const FONTSET: [u8; FONTSET_SIZE] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Debug)]
#[allow(dead_code)]
pub struct Cpu {
    memory: [u8; MEM_SIZE],
    v_reg: [u8; STACK_SIZE],
    stack: [u16; REG_SIZE],
    display: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    i_reg: u16,
    dt_reg: u8,
    st_reg: u8,
    pc: u16,
    sp: u8,
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            memory: [0; MEM_SIZE],
            v_reg: [0; STACK_SIZE],
            stack: [0; REG_SIZE],
            display: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            i_reg: 0,
            dt_reg: 0,
            st_reg: 0,
            pc: PC_START,
            sp: 0,
        };
        cpu.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        cpu
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = PC_START as usize;
        let end = start + data.len();
        self.memory[start..end].copy_from_slice(data);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.decode_and_execute(op);
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
            self.print_screen();
        }
    }

    fn print_screen(&self) {
        for y in self.display {
            for x in y {
                let pixel = if x { "*" } else { "." };
                print!("{pixel}");
            }
            println!();
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        let digit1 = (opcode & 0xF000) >> 12;
        let digit2 = (opcode & 0x0F00) >> 8;
        let digit3 = (opcode & 0x00F0) >> 4;
        let digit4 = opcode & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // 00E0 clear screen
            (0, 0, 0xE, 0) => {
                self.display = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
            }

            // 1NNN jump
            (1, _, _, _) => self.pc = opcode & 0x0FFF,

            // 6XNN Set register VX
            (6, _, _, _) => {
                let reg = (opcode & 0x0F00) >> 8;
                let num = opcode & 0x00FF;
                self.v_reg[reg as usize] = num as u8;
            }

            // 7XNN Add value to register VX
            (7, _, _, _) => {
                let reg = (opcode & 0x0F00) >> 8;
                let num = opcode & 0x00FF;
                self.v_reg[reg as usize] += num as u8;
            }

            // ANNN Set index register I
            (0xA, _, _, _) => {
                let num = opcode & 0x0FFF;
                self.i_reg = num;
            }

            // DXYN display
            // Deal with clipping later
            (0xD, x, y, n) => {
                let x_coord = self.v_reg[x as usize] as u16 % 64;
                let y_coord = self.v_reg[y as usize] as u16 % 32;
                self.v_reg[0xF] = 0;
                for y_cur in 0..n {
                    let pixel = self.memory[(self.i_reg + y_cur) as usize];
                    for x_cur in 0..8 {
                        let x = x_coord + x_cur;
                        let y = y_coord + y_cur;
                        let screen_pixel = self.display[y as usize][x as usize];
                        let sprite_pixel = pixel & (0b1000_0000 >> x_cur);
                        if sprite_pixel != 0 {
                            if screen_pixel {
                                self.v_reg[0xF] = 1;
                            }
                            self.display[y as usize][x as usize] ^= true;
                        }
                    }
                }
            }
            (_, _, _, _) => unimplemented!(),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
