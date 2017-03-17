
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
// use sdl2::mouse::Mouse;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

const DRAGGED_BORDER_COLOR: Color = Color::RGB(75, 75, 75);

pub struct DraggedShipUnit{
    x: i32,
    y: i32,
    ship_unit_index: Option<usize>
}

impl DraggedShipUnit {
    pub fn new() -> DraggedShipUnit{
        DraggedShipUnit{x:0, y:0, ship_unit_index:None}
    }

    pub fn manage_leftclicked_ship_unit(&mut self, ship_unit_index: Option<usize>, x: i32, y: i32){
        self.ship_unit_index=ship_unit_index;
        self.x=x;
        self.y=y;
    }

    pub fn manage_mouse_movement_while_leftclicked(&mut self, x: i32, y: i32){
        if self.ship_unit_index.is_some(){
            self.x=x;
            self.y=y;
        }
    }

    pub fn manage_unleftclick(&mut self) -> Option<usize>{
        if self.ship_unit_index.is_some(){
            let aux=self.ship_unit_index;
            self.ship_unit_index=None;
            aux
        } else {
            None
        }
    }
}

impl Drawable for DraggedShipUnit {
    fn draw(&self, gm: &mut GraphicsManager){
        if let Some(ship_unit_index)=self.ship_unit_index{
            gm.set_draw_color(DRAGGED_BORDER_COLOR);
            gm.fill_rect(Rect::new(self.x-16, self.y-16, 32, 32));
            gm.draw_ship_unit(ship_unit_index, self.x-14, self.y-14);
        }
    }
}