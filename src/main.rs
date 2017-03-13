#![feature(box_syntax)]
extern crate sdl2;
extern crate sdl2_image;

mod views;
mod fps_capper;
mod graphics_manager;
use views::View;
use fps_capper::FpsCapper;
use graphics_manager::GraphicsManager;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let timer = sdl_context.timer().unwrap();
    let mut fpscapper=FpsCapper::init(timer);
    // let image_context = sdl2_image::init(sdl2_image::INIT_PNG).unwrap();

    let window = video.window("Some Title", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    let mut event_pump=sdl_context.event_pump().unwrap();

    let mut gm=GraphicsManager::init(renderer);
    // gm.load_ship_units();

    let mut current_view=Some(View::viewa);
    loop{
        match current_view{
            Some(some_view) => {current_view=some_view.run(&mut event_pump, &mut gm, &mut fpscapper);},
            None => break,
        }
    }
}

