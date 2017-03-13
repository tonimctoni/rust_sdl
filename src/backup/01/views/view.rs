
use views::viewa::StructA;
use views::viewb::StructB;
use sdl2::EventPump;
use sdl2::render::Renderer;

pub enum View {
    StructA(StructA),
    StructB(StructB),
}

pub trait ViewTrait {
    fn run(self, event_pump: &mut EventPump, renderer: &mut Renderer) -> Option<View>;
}

impl ViewTrait for View {
    fn run(self, event_pump: &mut EventPump, renderer: &mut Renderer) -> Option<View>{
        return match self{
            View::StructA(x) => x.run(&mut event_pump, &mut renderer),
            View::StructB(x) => x.run(&mut event_pump, &mut renderer)
        };
    }
}
//remove the self in the views (make static)