
pub mod viewa;
// pub mod viewb;


use self::viewa::viewa;
use sdl2::EventPump;
use fps_capper::FpsCapper;
use graphics_manager::GraphicsManager;

macro_rules! create_view_enum {
    ($( $function_name:ident),*) => {
        #[allow(non_camel_case_types)]
        pub enum View{
            $($function_name),*
        }

        impl View{
            pub fn run(self, event_pump: &mut EventPump, gm: &mut GraphicsManager, fpscapper: &mut FpsCapper) -> Option<View>{
                match self{
                    $(View::$function_name=>$function_name(event_pump, gm, fpscapper)),*
                }
            }
        }
    }
}

create_view_enum!(viewa);

trait Drawable {
    fn draw(&self, &mut GraphicsManager);
}