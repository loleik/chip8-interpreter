use crate::opcodes::*;

use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs;

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

    return match instruction {
        0x00E0 => e_zero(emulator),
        0x00EE => double_e(emulator),
        _ if (instruction & 0xF000) == 0x1000 => one_nnn(instruction, emulator),

        _ if (instruction & 0xF000) == 0x6000 => six_x_kk(instruction, emulator),

        _ if (instruction & 0xF000) == 0xA000 => a_nnn(instruction, emulator),

        _ if (instruction & 0xF000) == 0xD000 => d_x_y_n(instruction, emulator),

        _ => unknown(instruction, emulator),
    };
}