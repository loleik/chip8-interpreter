use lolei_chip8::system::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Map sdl2 keycodes to chip8 keycodes.
fn map_keys(key: Keycode) -> Option<u8> {
    match key {
        Keycode::NUM_1 => Some(0x1),
        Keycode::NUM_2 => Some(0x2),
        Keycode::NUM_3 => Some(0x3),
        Keycode::NUM_4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // SDL2 variables for setting up the window and canvas.
    let sdl_content: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_content.video().unwrap();

    // Resolution is set to 640 x 320, which is 10x the original resolution for visibility.
    let window: Window = video_subsystem
                .window("Chip-8 Interpreter", 640, 320)
                .position_centered()
                .build()
                .unwrap();

    // Define the canvas which will be drawn to within the opcode functions.
    let canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump: sdl2::EventPump = sdl_content.event_pump().unwrap();

    // initialize the emulator struct with pass the ROM path and canvas.
    let mut emulator: Emulator = load("roms/5-quirks.ch8", canvas);

    // Used for tracking cycles. Not really needed past the first few test ROMs.
    let mut _cycles: i32 = 0;

    // Lays out the table for opcodes that's printed in the background.
    println!("┏━━━━━━┯━━━━━━━━━━━┯━━━━━━━━━━━┓");
    println!("┃Opcode│Instruction│Data       ┃");
    println!("┠──────┼───────────┼───────────┨ ");

    // Main loop, labeled for breaking on ESC.
    'running: loop {
        //emulator.memory[0x1FF] = 1;

        // Even pump for checking keypresses.
        if let Some(event) =  event_pump.poll_event() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(index) = map_keys(key) {
                        emulator.keypad[index as usize] = true;
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(index) = map_keys(key) {
                        emulator.keypad[index as usize] = false;
                    }
                },
                _ => {}
            }
        }

        // Fetch the instruction, and pass it to the decode function along with the emulator.
        let instruction: u16 = fetch(&mut emulator);
        decode(&mut emulator, instruction);
        
        update_timers(&mut emulator); // Update the timers.

        if emulator.vram_updated {
            for col in 0..64 {
                for row in 0..32 {
                    if emulator.display[row][col] == 1 {
                        let rect = Rect::new(
                            (col * 10) as i32,
                            (row * 10) as i32, 
                            10,
                            10);
                        emulator.canvas.set_draw_color(Color::RGB(255, 179, 71));
                        emulator.canvas.fill_rect(rect).unwrap();
                    }
                }
            }

            emulator.canvas.present();
            emulator.vram_updated = false;
        }

        // Again, not really needed outside of the first few test ROMs.
        //cycles += 1;

        /*if cycles == 20 {
            break;
        }*/
    }

    println!("┗━━━━━━┷━━━━━━━━━━━┷━━━━━━━━━━━┛");

    Ok(())
}
