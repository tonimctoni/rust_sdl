
use graphics_manager::GraphicsManager;
use views::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

pub struct ShipTray {
    x: i32,
    y: i32,
    ship_index: usize,
}

impl ShipTray {
    pub fn new(x: i32, y: i32, ship_index: usize) -> ShipTray{
        assert!(ship_index<5);
        ShipTray{x:x, y:y, ship_index:ship_index}
    }

    pub fn set_ship_index(&mut self, ship_index: usize){
        self.ship_index=ship_index;
    }

    pub fn get_ship_at(&self, x: i32, y: i32) -> Option<usize>{
        if x<self.x+2 || x>self.x+80-4{
            return None;
        }

        for i in 0..5{
            if y>=self.y+80*(i as i32)+2 && y<=self.y+80*((i as i32)+1)-4{
                return Some(i);
            }
        }

        None
    }
}

impl Drawable for ShipTray {
    fn draw(&self, gm: &mut GraphicsManager){
        gm.set_draw_color(Color::RGB(0, 127, 0));
        gm.fill_rect(Rect::new(self.x, self.y+80*(self.ship_index as i32), 80, 80));
        gm.set_draw_color(Color::RGB(0, 0, 0));
        gm.fill_rect(Rect::new(self.x+2, self.y+80*(self.ship_index as i32)+2, 80-4, 80-4));
        for i in 0..5{
            gm.draw_ship_icon(i, self.x, self.y+80*(i as i32));
        }
    }
}