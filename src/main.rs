mod system;

use crate::system::declare::type_def::{Byte, SignedByte, Word, SignedWord};
use crate::system::cartridge::{create_cartridge, Cartridge};


extern crate sdl2;

fn main()  {
    println!("Hello, world!" );

    let byte_temp : Byte = 255;
    let char_temp : SignedByte = 'c';
    let word_temp : Word = 20002;
    let signed_word_temp : SignedWord = 10001;

    println!("Byte? {}" ,byte_temp);
    println!("SignedByte {}", char_temp);
    println!("Word {}", word_temp);
    println!("SignedWord {}", signed_word_temp);

    let result_cartridge = create_cartridge();

    let cartridge = match result_cartridge // Rust의 Switch Case.
    {
        Ok(result_cartridge) => result_cartridge,
        Err(result_cartridge) => return
    };

    cartridge.load_memory();
}


/*
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
*/