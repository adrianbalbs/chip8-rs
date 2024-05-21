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
struct Cpu {
    memory: [u8; MEM_SIZE],
    v_reg: [u8; STACK_SIZE],
    stack: [u16; REG_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
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
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            i_reg: 0,
            dt_reg: 0,
            st_reg: 0,
            pc: PC_START,
            sp: 0,
        };
        cpu.memory[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        cpu
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}
