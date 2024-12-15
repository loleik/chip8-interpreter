use crate::opcodes::*;

use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs;

pub const FONT: [u8; 80] = [
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

pub struct Emulator {
    pub memory: [u8; 4096],
    pub pc: u16,
    pub sp: u8,
    pub vx: [u8; 16],
    pub i: u16,
    pub delay: u8,
    pub sound: u8,
    pub stack: [u16; 16],
    pub display: [[u8; 64]; 32],
    pub canvas: Canvas<Window>
}

impl Emulator {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self {
            memory: [0; 4096],
            pc: 0x0200,
            sp: 0,
            vx: [0; 16],
            i: 0,
            delay: 0,
            sound: 0,
            stack: [0; 16],
            display: [[0; 64]; 32],
            canvas: canvas
        }
    }
}

pub fn load(path: &str, canvas: Canvas<Window>) -> Emulator {
    let mut emulator: Emulator = Emulator::new(canvas);
    let data: Vec<u8> = match fs::read(path) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening file: {error:?}")
    };

    //emulator.memory[0x0050..0x009F].copy_from_slice(&FONT);

    emulator.memory[0x0200..0x0200 + data.len()].copy_from_slice(&data);

    emulator
}

pub fn fetch(emulator: &mut Emulator) -> u16 {
    let instruction: u16 = (emulator.memory[emulator.pc as usize] as u16) << 8
                         | (emulator.memory[emulator.pc as usize + 1] as u16);

    instruction
}

pub fn decode(
    emulator: &mut Emulator, 
    instruction: u16,
) -> &mut Emulator {
    emulator.pc += 2;

    let prefix: u16 = instruction & 0xF000;
    let suffix: u16 = instruction & 0x00FF;
    let least_significant: u16 = instruction & 0x000F;

    return match prefix {
        0x0000 => {
            match suffix {
                0x00E0 => e_0(emulator),
                0x00EE => e_e(emulator),
                _ => unknown(instruction, emulator),
            }
        }
        0x1000 => one_nnn(instruction, emulator),
        0x2000 => two_nnn(instruction, emulator),
        0x3000 => three_x_kk(instruction, emulator),
        0x4000 => four_x_kk(instruction, emulator),
        0x5000 => five_x_y_0(instruction, emulator),
        0x6000 => six_x_kk(instruction, emulator),
        0x7000 => seven_x_kk(instruction, emulator),
        0x8000 => {
            match least_significant {
                0x0001 => eight_x_y_1(instruction, emulator),
                0x0002 => eight_x_y_2(instruction, emulator),
                0x0003 => eight_x_y_3(instruction, emulator),
                0x0004 => eight_x_y_4(instruction, emulator),
                0x0005 => eight_x_y_5(instruction, emulator),
                0x0006 => eight_x_y_6(instruction, emulator),
                0x0007 => eight_x_y_7(instruction, emulator),
                0x000E => eight_x_y_e(instruction, emulator),
                _ => unknown(instruction, emulator),
            }
        }
        0x9000 => nine_x_y_0(instruction, emulator),
        0xA000 => a_nnn(instruction, emulator),

        0xD000 => d_x_y_n(instruction, emulator),

        0xF000 => {
            match suffix {
                0x001E => f_x_1e(instruction, emulator),
                0x0033 => f_x_33(instruction, emulator),
                0x0055 => f_x_55(instruction, emulator),
                0x0065 => f_x_65(instruction, emulator),
                _ => unknown(instruction, emulator),
            }
        }
        _ => unknown(instruction, emulator),
    };
}