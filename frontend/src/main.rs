use std::{env, fs::File, io::Read};

use chip8_core::emulator::Cpu;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas};

const SCALE: u32 = 15;
const SCREEN_WIDTH: u32 = 64 * SCALE;
const SCREEN_HEIGHT: u32 = 32 * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    let mut cpu = Cpu::init(&buffer);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip8-RS", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for _ in 0..10 {
            cpu.tick();
        }
        cpu.decrement_timers();
        draw_screen(&cpu, &mut canvas);
    }
}

fn draw_screen(cpu: &Cpu, canvas: &mut Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let screen = cpu.get_display();

    canvas.set_draw_color(Color::WHITE);
    for (y, row) in screen.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let x = x as u32;
            let y = y as u32;
            if *col {
                canvas
                    .fill_rect(Rect::new(
                        (x * SCALE) as i32,
                        (y * SCALE) as i32,
                        SCALE,
                        SCALE,
                    ))
                    .unwrap();
            }
        }
    }
    canvas.present();
}
