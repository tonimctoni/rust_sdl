
use views::view::View;
use views::view::ViewTrait;
use sdl2::EventPump;
use sdl2::render::Renderer;
// use sdl2::pixels::Color;

pub struct StructB {
}

impl ViewTrait for StructB {
    fn run(self, event_pump: &mut EventPump, renderer: &mut Renderer) -> Option<View>{
        return None;
    }
}