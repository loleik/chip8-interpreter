use crate::system::Emulator;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread;

// Clears both the canvas and display array.
pub fn e_0(emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ 00E0 │ CLS       │           ┃");

    for y in 0..emulator.display.len() {
        for x in 0..emulator.display[0].len() {
            emulator.display[y][x] = 0
        }
    }

    emulator.canvas.clear();

    emulator
}

// Returns from subroutine.
pub fn e_e(emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ 00EE │ RET       │           ┃");

    emulator.pc = emulator.stack[emulator.sp as usize];
    emulator.sp -= 1;

    emulator
}

// 0x0nnn is ignored as it isn't needed.
// Jumps to a location in memory.
pub fn one_nnn(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let nnn: u16 = opcode & 0x0FFF;

    println!("┃ {opcode:04X} │ JP        │ {nnn:03X}       ┃");

    emulator.pc = nnn;
    emulator
}

// Calls a subroutine.
pub fn two_nnn(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let nnn: u16 = opcode & 0x0FFF;
    
    println!("┃ {opcode:04X} │ CALL      │ {nnn:03X}       ┃");

    emulator.sp += 1;
    emulator.stack[emulator.sp as usize] = emulator.pc;
    emulator.pc = nnn;

    emulator
}

// Skips next instruction if Vx = kk
pub fn three_x_kk(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let kk: u16 = opcode & 0x00FF;

    println!("┃ {opcode:04X} │ SE        │ V{x:01X}, {kk:02X}    ┃");

    if emulator.vx[x] == kk as u8 {
        emulator.pc += 2
    }

    emulator
}

// Skip next instruction if Vx != kk.
pub fn four_x_kk(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let kk: u16 = opcode & 0x00FF;

    println!("┃ {opcode:04X} │ SNE       │ V{x:01X}, {kk:02X}    ┃");

    if emulator.vx[x] != kk as u8 {
        emulator.pc += 2
    }

    emulator
}

// Skip next instruction if Vx = Vy.
pub fn five_x_y_0(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SE        │ V{x:01X}, V{y:01X}    ┃");

    if emulator.vx[x] == emulator.vx[y] {
        emulator.pc += 2
    }

    emulator
}

// Set Vx = kk
pub fn six_x_kk(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: u16 = (opcode & 0x0F00) >> 8;
    let kk: u16 = opcode & 0x00FF;

    println!("┃ {opcode:04X} │ LD        │ V{x:01X}, {kk:02X}    ┃");

    emulator.vx[x as usize] = kk as u8;
    emulator
}

// Set Vx = Vx + kk.
pub fn seven_x_kk(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let kk: u16 = opcode & 0x00FF;

    emulator.vx[x] = emulator.vx[x].overflowing_add(kk as u8).0;

    emulator
}

// Set Vx = Vy.
pub fn eight_x_y_0(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ LD        │ V{x:01X}, V{y:01X}    ┃");

    emulator.vx[x] = emulator.vx[y];

    emulator
}

// Set Vx = Vx OR Vy.
pub fn eight_x_y_1(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ OR        │ V{x:01X}, V{y:01X}   ┃");

    emulator.vx[x] = emulator.vx[x] | emulator.vx[y];

    emulator
}

// Set Vx = Vx AND Vy.
pub fn eight_x_y_2(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ AND       │ V{x:01X}, V{y:01X}   ┃");

    emulator.vx[x] = emulator.vx[x] & emulator.vx[y];

    emulator}

// Set Vx = Vx XOR Vy.
pub fn eight_x_y_3(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ XOR       │ V{x:01X}, V{y:01X}   ┃");

    emulator.vx[x] = emulator.vx[x] ^ emulator.vx[y];

    emulator
}

// Set Vx = Vx + Vy, set VF = carry.
pub fn eight_x_y_4(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ ADD       │ V{x:01X}, V{y:01X}   ┃");

    let result: (u8, bool) = emulator.vx[x].overflowing_add(emulator.vx[y]);

    emulator.vx[x] = result.0;

    if result.1 {
        emulator.vx[15] = 1
    } else {
        emulator.vx[15] = 0
    }

    emulator
}

// Set Vx = Vx - Vy, set VF = NOT borrow.
pub fn eight_x_y_5(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SUB       │ V{x:01X}, V{y:01X}   ┃");

    let result: (u8, bool) = emulator.vx[x].overflowing_sub(emulator.vx[y]);

    emulator.vx[x] = result.0;

    if !result.1 {
        emulator.vx[15] = 1
    } else {
        emulator.vx[15] = 0
    }

    emulator
}

// Set Vx = Vx SHR 1.
pub fn eight_x_y_6(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SHR       │ V{x:01X} {{, V{y:01X}}} ┃");

    let lsb: u8 =  emulator.vx[x] & 0b1;

    emulator.vx[x] = emulator.vx[x] >> 1;

    emulator.vx[0xF] = lsb;

    emulator
}

// Set Vx = Vy - Vx, set VF = NOT borrow.
pub fn eight_x_y_7(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SUBN      │ V{x:01X}, V{y:01X}   ┃");

    let result: (u8, bool) = emulator.vx[y].overflowing_sub(emulator.vx[x]);

    emulator.vx[x] = result.0;

    if !result.1 {
        emulator.vx[15] = 1
    } else {
        emulator.vx[15] = 0
    }

    emulator
}

// Set Vx = Vx SHL 1.
pub fn eight_x_y_e(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SHL       │ V{x:01X} {{, V{y:01X}}} ┃");

    let msb: u8 = emulator.vx[x] >> 7 & 1;

    emulator.vx[x] = emulator.vx[x] << 1;

    emulator.vx[0xF] = msb;

    emulator
}

// Skip next instruction if Vx != Vy.
pub fn nine_x_y_0(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;
    let y: usize = ((opcode & 0x00F0) >> 4) as usize;

    println!("┃ {opcode:04X} │ SNE       │ V{x:01X}, V{y:01X}    ┃");

    if emulator.vx[x] != emulator.vx[y] {
        emulator.pc += 2
    }

    emulator
}

// Set I = nnn.
pub fn a_nnn(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let nnn: u16 = opcode & 0x0FFF;

    println!("┃ {opcode:04X} │ LD        │ I, {nnn:03X}    ┃");

    emulator.i = nnn;
    emulator
}

// Jump to location nnn + V0.

// Set Vx = random byte AND kk.

// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
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
    emulator.canvas.set_draw_color(Color::RGB(255, 179, 71));
    emulator.canvas.present();

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

// Skip next instruction if key with the value of Vx is pressed.

// Skip next instruction if key with the value of Vx is not pressed.

// Set Vx = delay timer value.
pub fn f_x_07(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    emulator.vx[x] = emulator.delay;
    
    emulator
}

// Set delay timer = Vx.
pub fn f_x_15(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    emulator.delay = emulator.vx[x];

    emulator
}

// Set sound timer = Vx.
pub fn f_x_18(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    emulator.sound = emulator.vx[x];

    emulator
}

// Set I = I + Vx.
pub fn f_x_1e(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    emulator.i += emulator.vx[x] as u16;

    emulator
}

// Set I = location of sprite for digit Vx.
/*pub fn f_x_29(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ {opcode:04X} │ LD        │ F, V{x:01X}      ┃");



    emulator
}*/

// Store BCD representation of Vx in memory locations I, I+1, and I+2.
pub fn f_x_33(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    println!("┃ {opcode:04X} │ LD        │ B, V{x:01X}      ┃");

    emulator.memory[emulator.i as usize] = emulator.vx[x] / 100;
    emulator.memory[emulator.i as usize + 1] = (emulator.vx[x] % 100) / 10;
    emulator.memory[emulator.i as usize + 2] = emulator.vx[x] % 10;

    emulator
}

// Store registers V0 through Vx in memory starting at location I.
pub fn f_x_55(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    println!("┃ {opcode:04X} │ LD        │ [I], V{x:01X}    ┃");

    for n in 0..=x {
        emulator.memory[emulator.i as usize + n] = emulator.vx[n];
    }

    emulator
}

// Read registers V0 through Vx from memory starting at location I.
pub fn f_x_65(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    let x: usize = ((opcode & 0x0F00) >> 8) as usize;

    println!("┃ {opcode:04X} │ LD        │ V{x:01X}, [I]    ┃");

    for n in 0..=x {
        emulator.vx[n] = emulator.memory[emulator.i as usize + n]
    }

    emulator
}

// Placeholder for unknown instructions.
pub fn unknown(opcode: u16, emulator: &mut Emulator) -> &mut Emulator {
    println!("┃ {opcode:04X} │ UNKNOWN   │           ┃");


    emulator
}