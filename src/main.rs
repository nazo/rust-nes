extern crate sdl2;

use std::time::Duration;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod rom;
mod cpu;
mod cpu_memory;
mod ppu;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("nes", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let nes_rom = rom::load_nes("sample1.nes");
    // let nes_rom = rom::load_nes("nestest.nes");
    let mut cpu = cpu::new_cpu();
    let mut ppu = ppu::new_ppu(&nes_rom.character_rom.data);
    let mut mem = cpu_memory::new_memory(&nes_rom.program_rom.data, &mut ppu);

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, 256, 256).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut v_canvas = vec![0; 256*256*3];

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                _ => {}
            }
        }

        println!("---");
        cpu::run(&mut cpu, &mut mem);

        ppu::draw_to_canvas(&mut v_canvas, &mem.ppu);

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let v_offset = y*(256*3) + x * 3;
                    let offset = y*pitch + x*3;
                    buffer[offset] = v_canvas[v_offset];
                    buffer[offset + 1] = v_canvas[v_offset + 1];
                    buffer[offset + 2] = v_canvas[v_offset + 2];
                }
            }
        }).unwrap();

        canvas.clear();
        canvas.copy(&texture, Some(Rect::new(0, 0, 256, 256)), Some(Rect::new(0, 0, 512, 512))).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}