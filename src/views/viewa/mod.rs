
mod ship_unit_tray;
mod ship_dock;
mod dragged_ship_unit;
mod ship_unit_socket_coords;
mod text_input;
mod button;
mod error_message;
mod ship_list;

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
use self::ship_list::ShipList;

pub fn viewa(event_pump: &mut EventPump, gm: &mut GraphicsManager, tm: &mut TimeManager) -> Option<View>{
    let shipunittray=ShipUnitTray::new(280,10);
    let mut shipdock=ShipDock::new(280,50,2);
    let mut draggedshipunit=DraggedShipUnit::new();
    let mut textinput=TextInput::new(10+280, 475, gm);
    let mut ok_button=Button::new(10+280+256+5,475+3,0);
    let mut clear_button=Button::new(580, 14 ,1);
    let mut errormessage=ErrorMessage::new(350, 580, gm);
    let mut shiplist=ShipList::new(10,50, gm);
    // shiplist.add_ship("Some name".to_string(), [None;32], 1, gm);
    // shiplist.add_ship("Some name".to_string(), [None;32], 1, gm);
    // shiplist.add_ship("Some name".to_string(), [None;32], 1, gm);
    // shiplist.add_ship("Some name".to_string(), [None;32], 1, gm);
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
                            shipdock.manage_leftclick(x, y);
                            draggedshipunit.manage_leftclicked_ship_unit(shipunittray.get_ship_unit_index_at(x, y).or(shipdock.manage_leftclicked_ship_unit(x, y)) , x, y);
                            textinput.manage_leftclick(x, y);
                            ok_button.manage_leftclick(x, y);
                            clear_button.manage_leftclick(x, y);
                            shiplist.manage_leftclick(x, y);
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
                            if ok_button.manage_unleftclick(x, y){
                                let text=textinput.get_text();
                                if text.is_empty(){
                                    errormessage.set_message("Need to specify a name for the ship.", gm);
                                }else{
                                    match shiplist.add_ship(textinput.get_text(), shipdock.get_ship_unit_sockets(), shipdock.get_ship_index(), gm) {
                                        Some(x) => errormessage.set_message(x, gm),
                                        None => {
                                            shipdock.clear_ship_unit_sockets();
                                            textinput.clear_text(gm);
                                        }
                                    }
                                }
                            }
                            if clear_button.manage_unleftclick(x, y){
                                shipdock.clear_ship_unit_sockets();
                            }
                            if let Some(x)=shiplist.manage_unleftclick(x, y){
                                shipdock.set_ship_unit_sockets_and_ship_index(x);
                            }
                        }
                        _ => {}
                    }
                }

                // TextEditing{text, start, length, ..} => {
                //     println!("TextEditing -> {:?}, {:?}, {:?}", text, start, length);
                // }

                TextInput{text, ..} => {
                    if !textinput.manage_text_input(text, gm){
                        errormessage.set_message("Ship's name may only be up to 10 characters long.", gm);
                    }
                }

                _ => {}
            }
        }
        //Further logic part
        errormessage.manage_frame_pass();

        // if clear_button.was_pressed(){
        //     shipdock.clear_ship_unit_sockets();
        // }

        // if ok_button.was_pressed(){
        //     let text=textinput.get_text();
        //     if text.is_empty(){
        //         errormessage.set_message("Need to specify a name for the ship.", gm);
        //     }else{
        //         match shiplist.add_ship(textinput.get_text(), shipdock.get_ship_unit_sockets(), shipdock.get_ship_index(), gm) {
        //             Some(x) => errormessage.set_message(x, gm),
        //             None => {
        //                 shipdock.clear_ship_unit_sockets();
        //                 textinput.clear_text(gm);
        //             }
        //         }
        //     }
        // }

        //Drawing part
        gm.set_draw_color(Color::RGB(0, 0, 0));
        gm.clear();
        shipunittray.draw(gm);
        shipdock.draw(gm);
        draggedshipunit.draw(gm);
        textinput.draw(gm);
        ok_button.draw(gm);
        clear_button.draw(gm);
        errormessage.draw(gm);
        shiplist.draw(gm);
        tm.cap_fps();
        gm.present();
    }
}
