
mod ship_unit_tray;
mod ship_dock;
mod dragged_ship_unit;
mod ship_unit_socket_coords;
mod text_input;
mod button;

use views::View;
use sdl2::EventPump;
use sdl2::pixels::Color;
use fps_capper::FpsCapper;
use graphics_manager::GraphicsManager;
use self::ship_unit_tray::ShipUnitTray;
use self::ship_dock::ShipDock;
use self::dragged_ship_unit::DraggedShipUnit;
use views::Drawable;
use self::text_input::TextInput;
use self::button::Button;

pub fn viewa(event_pump: &mut EventPump, gm: &mut GraphicsManager, fpscapper: &mut FpsCapper) -> Option<View>{
    let shipunittray=ShipUnitTray::new(10,10);
    let mut shipdock=ShipDock::new(10,50,0);
    let mut draggedshipunit=DraggedShipUnit::new();
    let mut textinput=TextInput::new(10, 475, 0);
    let mut ok_button=Button::new(10+256+5,475,0);
    loop{
        for event in event_pump.poll_iter(){
            use sdl2::event::Event::*;
            match event {
                Quit { .. } => return None,

                KeyDown { keycode, .. } => {
                    use sdl2::keyboard::Keycode::*;
                    match keycode {
                        Some(Escape) => return None,
                        Some(Backspace) => textinput.manage_backspace_press(gm),
                        _ => {}
                    }
                }

                MouseButtonDown{mouse_btn, x, y, ..} => {
                    use sdl2::mouse::Mouse::*;
                    match mouse_btn{
                        Left => {
                            shipdock.manage_left_click(x, y);
                            draggedshipunit.manage_leftclicked_ship_unit(shipunittray.get_ship_unit_index_at(x, y).or(shipdock.manage_leftclicked_ship_unit(x, y)) , x, y);
                            textinput.manage_left_click(x, y);
                            ok_button.manage_left_click(x, y);
                        }

                        Right => {
                            shipdock.add_ship_unit(shipunittray.get_ship_unit_index_at(x, y));
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
                            // draggedshipunit.manage_unleftclick();
                            shipdock.manage_unleftclicked_ship_unit(draggedshipunit.manage_unleftclick(), x, y);
                            ok_button.manage_unleft_click(x, y);
                        }
                        _ => {}
                    }
                }

                // TextEditing{text, start, length, ..} => {
                //     println!("TextEditing -> {:?}, {:?}, {:?}", text, start, length);
                // }

                TextInput{text, ..} => {
                    println!("TextInput -> {:?}", text);
                    textinput.manage_text_input(text, gm)
                }

                _ => {}
            }
        }
        gm.set_draw_color(Color::RGB(0, 0, 0));
        gm.clear();
        shipunittray.draw(gm);
        shipdock.draw(gm);
        draggedshipunit.draw(gm);
        textinput.draw(gm);
        ok_button.draw(gm);
        fpscapper.cap();
        gm.present();
    }
}
