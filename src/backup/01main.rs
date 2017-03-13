extern crate sdl2;nts;

use sdl2::pixels::Color;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("Some Title", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let mut event_pump=sdl_context.event_pump().unwrap();

    let mut key_escape=false;
    'outer_loop: loop {
        for event in event_pump.poll_iter(){
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => {break 'outer_loop},

                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => key_escape = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => key_escape = false,
                    _ => {}
                },

                _ => {}
            }
        }

        if key_escape {
            break;
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}

