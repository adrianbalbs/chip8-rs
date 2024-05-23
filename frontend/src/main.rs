use std::{env, fs::File, io::Read};

use chip8_core::emulator::Cpu;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }
    let mut cpu = Cpu::new();
    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    cpu.load(&buffer);
    cpu.run();
}
