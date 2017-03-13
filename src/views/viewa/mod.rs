
mod ship_unit_tray;
mod ship_dock;
mod dragged_ship_unit;
mod ship_unit_socket_coords;

use views::View;
use sdl2::EventPump;
use sdl2::pixels::Color;
use fps_capper::FpsCapper;
use graphics_manager::GraphicsManager;
use self::ship_unit_tray::ShipUnitTray;
use self::ship_dock::ShipDock;
use self::dragged_ship_unit::DraggedShipUnit;
use views::Drawable;


pub fn viewa(event_pump: &mut EventPump, gm: &mut GraphicsManager, fpscapper: &mut FpsCapper) -> Option<View>{
    let shipunittray=ShipUnitTray::new(10,10);
    let mut shipdock=ShipDock::new(10,50,0);
    let mut draggedshipunit=DraggedShipUnit::new();
    loop{
        for event in event_pump.poll_iter(){
            use sdl2::event::Event::*;
            match event {
                Quit { .. } => return None,

                KeyDown { keycode, .. } => {
                    use sdl2::keyboard::Keycode::*;
                    match keycode {
                        Some(Escape) => return None,
                        _ => {}
                    }
                }

                MouseButtonDown{mouse_btn, x, y, ..} => {
                    use sdl2::mouse::Mouse::*;
                    match mouse_btn{
                        Left => {
                            shipdock.manage_left_click(x, y);
                            //for several ship_unit sources might want to use "or"
                            draggedshipunit.manage_leftclicked_ship_unit(shipunittray.get_ship_unit_index_at(x, y) , x, y);
                        }
                        _ => {}
                    }
                }

                MouseMotion{mousestate, x, y, ..} => {
                    if mousestate.left(){
                        draggedshipunit.manage_mouse_movement_while_leftclicked(x,y);
                    }
                }

                MouseButtonUp{mouse_btn, x, y, ..} => {
                    use sdl2::mouse::Mouse::*;
                    match mouse_btn{
                        Left => {
                            draggedshipunit.manage_unleftclick();
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
        gm.set_draw_color(Color::RGB(0, 0, 0));
        gm.clear();
        shipunittray.draw(gm);
        shipdock.draw(gm);
        draggedshipunit.draw(gm);
        // shiptray.draw(gm);
        // gm.set_draw_color(Color::RGB(127, 0, 155));
        // gm.fill_rect(Rect::new(8,8,32,32));
        // gm.draw_ship_unit(1,10,10);
        fpscapper.cap();
        gm.present();
    }
}
