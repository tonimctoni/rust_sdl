
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

const OUTER_BORDER_COLOR: Color = Color::RGB(75, 75, 75);
const UNIT_BORDER_COLOR: Color = Color::RGB(127, 0, 255);

pub struct ShipUnitTray {
    x: i32,
    y: i32,
}

impl ShipUnitTray {
    pub fn new(x: i32, y: i32) -> ShipUnitTray{
        ShipUnitTray{x:x, y:y}
    }

    pub fn get_ship_unit_index_at(&self, x: i32, y: i32) -> Option<usize>{
        if y<self.y+2 || y>self.y+2+32{//y<self.y+4 || y>self.y+32{
            return None
        }

        for i in 0..8{
            if x>self.x+2+36*(i as i32) && x<self.x+2+36*(i as i32)+32{//x>self.x+4+36*(i as i32) && x<self.x+4+36*(i as i32)+28{
                return Some(i);
            }
        }

        None
    }
}

impl Drawable for ShipUnitTray {
    fn draw(&self, gm: &mut GraphicsManager){
        gm.set_draw_color(OUTER_BORDER_COLOR);
        gm.draw_rect(Rect::new(self.x, self.y, 8*36, 36));
        gm.set_draw_color(UNIT_BORDER_COLOR);
        for i in 0..8{
            gm.fill_rect(Rect::new(self.x+2+36*(i as i32), self.y+2, 32, 32));
            gm.draw_ship_unit(i, self.x+4+36*(i as i32), self.y+4);
        }
    }
}