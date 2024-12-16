use lolei_chip8::system::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
        // Even pump for checking keypresses.
        if let Some(event) =  event_pump.poll_event() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Fetch the instruction, and pass it to the decode function along with the emulator.
        let instruction: u16 = fetch(&mut emulator);
        decode(&mut emulator, instruction);

        // Again, not really needed outside of the first few test ROMs.
        //cycles += 1;

        /*if cycles == 20 {
            break;
        }*/
    }

    println!("┗━━━━━━┷━━━━━━━━━━━┷━━━━━━━━━━━┛");

    Ok(())
}
