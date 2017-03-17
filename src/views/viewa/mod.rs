
mod ship_unit_tray;
mod ship_dock;
mod dragged_ship_unit;
mod ship_unit_socket_coords;
mod text_input;
mod button;
mod error_message;

use views::View;
use sdl2::EventPump;
use sdl2::pixels::Color;
use time_manager::TimeManager;
use graphics_manager::GraphicsManager;
use self::ship_unit_tray::ShipUnitTray;
use self::ship_dock::ShipDock;
use self::dragged_ship_unit::DraggedShipUnit;
use graphics_manager::Drawable;
use self::text_input::TextInput;
use self::button::Button;
use self::error_message::ErrorMessage;

pub fn viewa(event_pump: &mut EventPump, gm: &mut GraphicsManager, tm: &mut TimeManager) -> Option<View>{
    let shipunittray=ShipUnitTray::new(280,10);
    let mut shipdock=ShipDock::new(280,50,2);
    let mut draggedshipunit=DraggedShipUnit::new();
    let mut textinput=TextInput::new(10+280, 475, gm);
    let mut ok_button=Button::new(10+280+256+5,475+3,0);
    let mut errormessage=ErrorMessage::new(400, 580, gm);
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
                    if !textinput.manage_text_input(text, gm){
                        errormessage.set_message("Ships name may only be 10 characters long.", gm);
                    }
                }

                _ => {}
            }
        }
        //Further logic part
        errormessage.manage_frame_pass();

        //Drawing part
        gm.set_draw_color(Color::RGB(0, 0, 0));
        gm.clear();
        shipunittray.draw(gm);
        shipdock.draw(gm);
        draggedshipunit.draw(gm);
        textinput.draw(gm);
        ok_button.draw(gm);
        errormessage.draw(gm);
        tm.cap_fps();
        gm.present();
    }
}
