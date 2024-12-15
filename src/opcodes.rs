use crate::system::Emulator;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread;

pub fn e_zero(emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ 00E0 │ CLS       │           ┃");

    emulator
}

pub fn double_e(emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ 00EE │ RET       │           ┃");

    emulator
}

pub fn one_nnn(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let nnn: u16 = opcode & 0x0FFF;

    println!("┃ {opcode:04X} │ JP        │ {nnn:03X}       ┃");

    emulator.pc = nnn;
    emulator
}

/*pub fn two_nnn(opcode: u16) {
    let nnn: u16 = opcode & 0x0FFF;
    println!("{opcode:04X}: CALL {nnn:03X}")
}

pub fn three_x_kk(opcode: u16) {
    let x: u16 = opcode & 0x0F00;
    let kk: u16 = opcode & 0x00FF;
    println!("{opcode:04X}: SE V {x:02X}, {kk:02X}")
}*/

pub fn six_x_kk(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: u16 = (opcode & 0x0F00) >> 8;
    let kk: u16 = opcode & 0x00FF;

    println!("┃ {opcode:04X} │ LD        │ V{x:01X}, {kk:02X}    ┃");

    emulator.vx[x as usize] = kk as u8;
    emulator
}

pub fn a_nnn(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let nnn: u16 = opcode & 0x0FFF;

    println!("┃ {opcode:04X} │ LD        │ I, {nnn:03X}    ┃");

    emulator.i = nnn;
    emulator
}

pub fn d_x_y_n(
    opcode: u16, 
    emulator: &mut Emulator,
) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let vx: usize = emulator.vx[x] as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;
    let vy: usize = emulator.vx[y] as usize;
    let n: u16 = opcode & 0x000F;
    let i: u16 = emulator.i;

    emulator.canvas.set_draw_color(Color::RGB(255, 255, 255));

    println!("┃ {opcode:04X} │ DRW       │ V{x:01X}, V{y:01X}, {n:01X} ┃");

    let sprite: &[u8] = &emulator.memory[(i as usize)..((i + n) as usize)];
    
    for (row, &byte) in sprite.iter().enumerate() {
        let screen_y: usize = (vy + row) % 32;
        
        for bit in 0..8 {
            let screen_x: usize = (vx + bit) % 64;
            let sprite_bit: u8 = (byte >> (7 - bit )) & 1;

            if emulator.display[screen_y][screen_x] == 1 && sprite_bit == 1 {
                emulator.vx[15] = 1;
            }

            emulator.display[screen_y][screen_x] ^= sprite_bit;
        }
    }

    for col in 0..64 {
        for row in 0..32 {
            if emulator.display[row][col] == 1 {
                let rect = Rect::new(
                    (col * 10) as i32,
                    (row * 10) as i32, 
                    10,
                    10);
                emulator.canvas.fill_rect(rect).unwrap();
            }
        }
    }

    emulator.canvas.present(); // Update the screen
    
    thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    emulator
}

pub fn unknown(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ {opcode:04X} │ UNKNOWN   │           ┃");


    emulator
}