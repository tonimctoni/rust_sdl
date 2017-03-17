
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use views::viewa::ship_unit_socket_coords::SHIP_00_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_01_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_02_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_03_SOCKET_COORDS;
use views::viewa::ship_unit_socket_coords::SHIP_04_SOCKET_COORDS;

const OUTER_BORDER_COLOR: Color = Color::RGB(75, 75, 75);
const SELECTED_BORDER_COLOR: Color = Color::RGB(0, 127, 0);
const SELECTED_BKG_COLOR: Color = Color::RGB(0, 0, 0);
const SOCKET_BORDER_COLOR: Color = Color::RGB(0, 0, 0);
const SOCKET_BKG_COLOR: Color = Color::RGB(255, 255, 255);

pub struct ShipDock {
    x: i32,
    y: i32,
    ship_index: usize,
    ship_unit_sockets: [Option<usize>;32],
    ship_unit_socket_coords:  &'static [(i32,i32)]
}

impl ShipDock {
    fn ship_index_to_coords_ref(ship_index: usize) -> &'static [(i32,i32)]{
        match ship_index{
                        0 => &SHIP_00_SOCKET_COORDS,
                        1 => &SHIP_01_SOCKET_COORDS,
                        2 => &SHIP_02_SOCKET_COORDS,
                        3 => &SHIP_03_SOCKET_COORDS,
                        4 => &SHIP_04_SOCKET_COORDS,
                        _ => panic!("manage_left_click got a wrong ship_index"),
                    }
    }

    pub fn new(x: i32, y: i32, ship_index: usize) -> ShipDock{
        assert!(ship_index<5);
        ShipDock{x:x, y:y, ship_index:ship_index, ship_unit_sockets: [None;32], ship_unit_socket_coords: ShipDock::ship_index_to_coords_ref(ship_index)}
    }

    pub fn clear_sockets(&mut self){
        for su in self.ship_unit_sockets.iter_mut(){
            *su=None;
        }
    }

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
            self.ship_unit_socket_coords=ShipDock::ship_index_to_coords_ref(ship_index);
        }
    }

    pub fn manage_leftclicked_ship_unit(&mut self, x: i32, y: i32) -> Option<usize>{
        for (&(sx,sy), su) in self.ship_unit_socket_coords.iter().zip(self.ship_unit_sockets.iter_mut()){
            if x>self.x+sx && x<self.x+sx+32 && y>self.y+sy && y<self.y+sy+32{
                let aux=*su;
                *su=None;
                return aux;
            }
        }
        None
    }

    pub fn manage_unleftclicked_ship_unit(&mut self, ship_unit_index: Option<usize>, x: i32, y: i32){
        if ship_unit_index.is_some(){
            for (&(sx,sy), su) in self.ship_unit_socket_coords.iter().zip(self.ship_unit_sockets.iter_mut()){
                if x>self.x+sx && x<self.x+sx+32 && y>self.y+sy && y<self.y+sy+32{
                    *su=ship_unit_index;
                    break;
                }
            }
        }
    }

    pub fn add_ship_unit(&mut self, ship_unit_index: Option<usize>){
        if ship_unit_index.is_some(){
            for su in self.ship_unit_sockets.iter_mut().take(self.ship_unit_socket_coords.len()){
                if (*su).is_none(){
                    *su=ship_unit_index;
                    break;
                }
            }
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
        gm.set_draw_color(SOCKET_BORDER_COLOR);
        for &(sx,sy) in self.ship_unit_socket_coords.iter(){
            gm.fill_rect(Rect::new(self.x+sx, self.y+sy, 32, 32));
        }
        gm.set_draw_color(SOCKET_BKG_COLOR);
        for (&(sx,sy), &su) in self.ship_unit_socket_coords.iter().zip(self.ship_unit_sockets.iter()){
            match su{
                Some(su) => gm.draw_ship_unit(su, self.x+sx+2, self.y+sy+2),
                None => gm.fill_rect(Rect::new(self.x+sx+2, self.y+sy+2, 28, 28)),
            }
        }
    }
}