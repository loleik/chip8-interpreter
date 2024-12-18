use crate::opcodes::*;

use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fs;

// The common CHIP-8 font set.
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

// CHIP-8 resolution is 64 x 32.
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

// Emulalator struct (yes it's technically an interpreter but this isn't worth changing now).
pub struct Emulator {
    pub memory: [u8; 4096], // 4096 bytes of memory.
    pub pc: u16, // 16 bit program counter.
    pub sp: u8, // 8 bit stack pointer.
    pub vx: [u8; 16], // 8 bit V0-VF registers.
    pub i: u16, // 16 bit index counter.
    pub delay: u8, // 8 bit delay timer.
    pub sound: u8, // 8 bit sound timer.
    pub stack: [u16; 16], // 16 bit stack array.
    pub display: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], // 64 x 32 display array.
    pub keypad: [bool; 16], // 16 key CHIP-8 keypad array.
    pub canvas: Canvas<Window>, // Canvas for drawing during execution.
    pub vram_updated: bool, // Flag for drawing the screen.
    pub key_pressed: bool, // Flag for 0xFx0A.
}

// Creates a new emulator instance (again I know, interpreter haha).
// Only input that needs to be specified is the canvas.
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
            display: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keypad: [false; 16],
            canvas: canvas,
            vram_updated: false,
            key_pressed: false,
        }
    }
}

// Function for loading the emulator struct, then loading the ROM and font into memory.
pub fn load(path: &str, canvas: Canvas<Window>) -> Emulator {
    let mut emulator: Emulator = Emulator::new(canvas);

    let data: Vec<u8> = match fs::read(path) {
        Ok(data) => data,
        Err(error) => panic!("Problem opening file: {error:?}")
    };

    // Loads the font into unused memory.
    emulator.memory[0x0000..0x0050].copy_from_slice(&FONT);

    // See Cowgod's technical reference for the memory.
    emulator.memory[0x0200..0x0200 + data.len()].copy_from_slice(&data);

    emulator
}

// Fetching next instruction from memory.
pub fn fetch(emulator: &mut Emulator) -> u16 {
    // Instructions are two bytes long.
    // First byte is at the program counter value in memory.
    // Second byte is at the program counter value + 1 in memory.
    let instruction: u16 = (emulator.memory[emulator.pc as usize] as u16) << 8
                         | (emulator.memory[emulator.pc as usize + 1] as u16);

    instruction
}

// Function for decoding and running instructions.
pub fn decode(
    emulator: &mut Emulator, 
    instruction: u16,
) -> &mut Emulator {
    emulator.pc += 2; // Incrememnt the program counter for next instruction.

    // Common parts of instructions for matching.
    let most_significant: u16 = instruction & 0xF000;

    let suffix: u16 = instruction & 0x00FF;
    let least_significant: u16 = instruction & 0x000F;

    // Big match statement for passing instructions through to their respective functions.
    return match most_significant {
        0x0000 => {
            match instruction {
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
                0x0000 => eight_x_y_0(instruction, emulator),
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
        0xB000 => b_nnn(instruction, emulator),
        0xC000 => c_x_kk(instruction, emulator),
        0xD000 => d_x_y_n(instruction, emulator),
        0xE000 => {
            match suffix {
                0x009E => e_x_9e(instruction, emulator),
                0x00A1 => e_x_a1(instruction, emulator),
                _ => unknown(instruction, emulator),
            }
        }
        0xF000 => {
            match suffix {
                0x0007 => f_x_07(instruction, emulator),
                0x000A => f_x_0a(instruction, emulator),
                0x0015 => f_x_15(instruction, emulator),
                0x0018 => f_x_18(instruction, emulator),
                0x0029 => f_x_29(instruction, emulator),
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