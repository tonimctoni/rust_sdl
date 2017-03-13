
use views::view::View;
use views::view::ViewTrait;
use sdl2::EventPump;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

pub struct StructA {
}

impl ViewTrait for StructA {
    fn run(self, event_pump: &mut EventPump, renderer: &mut Renderer) -> Option<View>{
        loop{
            for event in event_pump.poll_iter(){
                use sdl2::event::Event::*;
                use sdl2::keyboard::Keycode::*;

                match event {
                    Quit { .. } => return None,

                    KeyDown { keycode, .. } => match keycode {
                        Some(Escape) => return None,
                        _ => {}
                    },

                    // KeyUp { keycode, .. } => match keycode {
                    //     Some(Escape) => key_escape = false,
                    //     _ => {}
                    // },

                    _ => {}
                }
            }
            renderer.set_draw_color(Color::RGB(0, 0, 0));
            renderer.clear();
            renderer.present();
        }
    }
}