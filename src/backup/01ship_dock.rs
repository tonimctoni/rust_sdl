
use graphics_manager::GraphicsManager;
use views::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use views::viewa::ship_unit_socket_coords::SHIP_00_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_01_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_02_SOCKET_COORDS;
// use sdl2::mouse::Mouse;

const OUTER_BORDER_COLOR: Color = Color::RGB(75, 75, 75);
const SELECTED_BORDER_COLOR: Color = Color::RGB(0, 127, 0);
const SELECTED_BKG_COLOR: Color = Color::RGB(0, 0, 0);
const SOCKET_BORDER_COLOR: Color = Color::RGB(0, 0, 0);
const SOCKET_BKG_COLOR: Color = Color::RGB(255, 255, 255);

enum ShipUnitSockets {
    Ship00us([Option<usize>;12]),
    Ship01us([Option<usize>;32]),
    Ship02us([Option<usize>;4]),
}

impl ShipUnitSockets {
    fn new(ship_index: usize) -> ShipUnitSockets{
        match ship_index{
            0 => ShipUnitSockets::Ship00us([None;12]),
            1 => ShipUnitSockets::Ship01us([None;32]),
            2 => ShipUnitSockets::Ship02us([None;4]),
            _ => panic!("Initialized ShipUnitSockets with wrong ship_index")
        }
    }

    fn draw_at(&self, gm: &mut GraphicsManager, x: i32, y: i32){
        match self{
            &ShipUnitSockets::Ship00us(ref ship_units) => {
                gm.set_draw_color(SOCKET_BORDER_COLOR);
                for &(sx,sy) in SHIP_00_SOCKET_COORDS.iter(){
                    gm.fill_rect(Rect::new(x+sx, y+sy, 32, 32));
                }
                gm.set_draw_color(SOCKET_BKG_COLOR);
                for (&(sx,sy), &su) in SHIP_00_SOCKET_COORDS.iter().zip(ship_units.iter()){
                    match su{
                        Some(su) => gm.draw_ship_unit(su, x+sx+2, y+sy+2),
                        None => gm.fill_rect(Rect::new(x+sx+2, y+sy+2, 28, 28)),
                    }
                }
            }
            &ShipUnitSockets::Ship01us(ref ship_units) => {
                gm.set_draw_color(SOCKET_BORDER_COLOR);
                for &(sx,sy) in SHIP_01_SOCKET_COORDS.iter(){
                    gm.fill_rect(Rect::new(x+sx, y+sy, 32, 32));
                }
                gm.set_draw_color(SOCKET_BKG_COLOR);
                for (&(sx,sy), &su) in SHIP_01_SOCKET_COORDS.iter().zip(ship_units.iter()){
                    match su{
                        Some(su) => gm.draw_ship_unit(su, x+sx+2, y+sy+2),
                        None => gm.fill_rect(Rect::new(x+sx+2, y+sy+2, 28, 28)),
                    }
                }
            }
            &ShipUnitSockets::Ship02us(ref ship_units) => {
                gm.set_draw_color(SOCKET_BORDER_COLOR);
                for &(sx,sy) in SHIP_02_SOCKET_COORDS.iter(){
                    gm.fill_rect(Rect::new(x+sx, y+sy, 32, 32));
                }
                gm.set_draw_color(SOCKET_BKG_COLOR);
                for (&(sx,sy), &su) in SHIP_02_SOCKET_COORDS.iter().zip(ship_units.iter()){
                    match su{
                        Some(su) => gm.draw_ship_unit(su, x+sx+2, y+sy+2),
                        None => gm.fill_rect(Rect::new(x+sx+2, y+sy+2, 28, 28)),
                    }
                }
            }
        }
    }

    fn manage_unleftclicked_ship_unit(ship_unit_index: Option<usize>, x: i32, y: i32){
        // if let Some(ship_unit_index)=ship_unit_index{
        //     for (&(sx,sy), &su) in SHIP_00_SOCKET_COORDS.iter().zip(ship_units.iter()){
        //         su=Some(ship_unit_index);
        //     }
        // }
    }
}

pub struct ShipDock {
    x: i32,
    y: i32,
    ship_index: usize,
    ship_unit_sockets: ShipUnitSockets
}

impl ShipDock {
    pub fn new(x: i32, y: i32, ship_index: usize) -> ShipDock{
        assert!(ship_index<5);
        ShipDock{x:x, y:y, ship_index:ship_index, ship_unit_sockets: ShipUnitSockets::new(0)}
    }

    // pub fn manage_dropped_ship_unit(&mut self, ship_unit_index: Option<usize>, x: i32, y:i32){
    // }

    fn get_ship_icon_at(&self, x: i32, y: i32) -> Option<usize>{
        if x<self.x+407+2 || x>self.x+407+80-2{
            return None;
        }

        for i in 0..5{
            if y>=self.y+80*(i as i32)+2 && y<=self.y+80*((i as i32)+1)-2{
                return Some(i);
            }
        }

        None
    }

    pub fn manage_left_click(&mut self, x: i32, y: i32){
        let ship_index=self.get_ship_icon_at(x,y);
        if let Some(ship_index)=ship_index{
            self.ship_index=ship_index;
            self.ship_unit_sockets=ShipUnitSockets::new(ship_index);
        }
    }
}

impl Drawable for ShipDock {
    fn draw(&self, gm: &mut GraphicsManager){
        //Ship part
        gm.set_draw_color(OUTER_BORDER_COLOR);
        gm.draw_rect(Rect::new(self.x, self.y, 400+2, 400+2));
        gm.draw_ship(self.ship_index, self.x+1, self.y+1);

        //Tray part
        gm.set_draw_color(SELECTED_BORDER_COLOR);
        gm.fill_rect(Rect::new(self.x+407, self.y+80*(self.ship_index as i32), 80, 80));
        gm.set_draw_color(SELECTED_BKG_COLOR);
        gm.fill_rect(Rect::new(self.x+407+2, self.y+80*(self.ship_index as i32)+2, 80-4, 80-4));
        for i in 0..5{
            gm.draw_ship_icon(i, self.x+407, self.y+80*(i as i32));
        }

        //Sockets part
        self.ship_unit_sockets.draw_at(gm, self.x, self.y);
    }
}