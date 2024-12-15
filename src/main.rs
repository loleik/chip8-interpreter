use lolei_chip8::system::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let sdl_content: sdl2::Sdl = sdl2::init().unwrap();
    let video_subsystem: sdl2::VideoSubsystem = sdl_content.video().unwrap();

    let window: Window = video_subsystem.window("Chip-8 Interpreter", 640, 320)
                                            .position_centered()
                                            .build()
                                            .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump: sdl2::EventPump = sdl_content.event_pump().unwrap();

    let mut emulator: Emulator = load("1-chip8-logo.ch8", canvas);
    let mut cycles: i32 = 0;

    println!("┏━━━━━━┯━━━━━━━━━━━┯━━━━━━━━━━━┓");
    println!("┃Opcode│Instruction│Data       ┃");
    println!("┠──────┼───────────┼───────────┨ ");

    'running: loop { 
        emulator.canvas.set_draw_color(Color::RGB(0, 0, 0));

        if let Some(event) =  event_pump.poll_event() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        let instruction: u16 = fetch(&mut emulator);
        decode(&mut emulator, instruction);

        cycles += 1;

        if cycles == 39 {
            break;
        }
    }

    println!("┗━━━━━━┷━━━━━━━━━━━┷━━━━━━━━━━━┛");

    Ok(())
}
