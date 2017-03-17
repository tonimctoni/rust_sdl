
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use views::viewa::button::Button;

const OUTER_BORDER_COLOR: Color = Color::RGB(75, 75, 75);
const SHIP_NAME_COLOR: Color = Color::RGB(0, 255, 0);
const SELECTED_SHIP_COLOR: Color = Color::RGB(255, 0, 0);
const MAX_SHIPS: usize = 25;

//name, sockets, ship_index, text_index, button
struct ShipData(String, [Option<usize>;32], usize, usize, Button);

pub struct ShipList {
    x: i32,
    y: i32,
    ships: Vec<ShipData>,
    text_indices: Vec<usize>,
    clicked_ship: Option<usize>,
}

impl ShipList {
    pub fn new(x: i32, y: i32, gm: &mut GraphicsManager) -> ShipList{
        let mut text_indices=Vec::with_capacity(MAX_SHIPS);
        for _ in 0..MAX_SHIPS{
            text_indices.push(gm.get_new_text_index());
        }
        ShipList{x: x, y: y, ships:Vec::with_capacity(MAX_SHIPS), text_indices: text_indices, clicked_ship: None}
    }

    pub fn add_ship(&mut self, name: String, ship_unit_sockets: [Option<usize>;32], ship_index: usize, gm: &mut GraphicsManager) -> Option<&str>{
        for &ShipData(ref this_name, _, _, _, _) in self.ships.iter(){
            if *this_name==name{
                return Some("Unable to add more ships with that name.");
            }
        }
        match self.text_indices.pop(){
            Some(x) => {
                gm.set_text_with_bitstream_vera_16bd(x, &name, SHIP_NAME_COLOR);
                let new_index:i32=self.ships.len() as i32;
                self.ships.push(ShipData(name, ship_unit_sockets, ship_index, x, Button::new(self.x+5,self.y+5+20*new_index+2,2)));
                None
            }
            None => Some("Unable to add further ships."),
        }
    }

    fn get_list_index_at(&self, x: i32, y: i32) -> Option<usize>{
        for i in 0..(self.ships.len() as i32){
            if x>=self.x+5+18 && x<=self.x+5+18+128 && y>=self.y+5+20*i && y<=self.y+5+20*i+20{
                return Some(i as usize);
            }
        }
        None
    }

    pub fn manage_leftclick(&mut self, x: i32, y: i32){
        for &mut ShipData(_, _, _, _, ref mut btn) in self.ships.iter_mut(){
            btn.manage_leftclick(x,y);
        }

        self.clicked_ship=self.get_list_index_at(x, y);
    }

    pub fn manage_unleftclick(&mut self, x: i32, y: i32) -> Option<([Option<usize>;32], usize)>{

        let mut index_to_remove=None;
        for (i, &mut ShipData(_, _, _, _, ref mut btn)) in self.ships.iter_mut().enumerate(){
            if btn.manage_unleftclick(x,y){
                index_to_remove=Some(i);
                break;
            }
        }

        if let Some(i)=index_to_remove{
            self.text_indices.push(self.ships[i].3);
            self.ships.remove(i);
            for (&mut ShipData(_, _, _, _, ref mut btn), i) in self.ships.iter_mut().zip(0..).skip(i){
                btn.set_pos(self.x+5, self.y+5+20*i+2);
            }
            // Button::new(self.x+5,self.y+5+20*new_index+2,2)
        }

        if let Some(ship_at_xy)=self.get_list_index_at(x, y){
            if let Some(clicked_ship)=self.clicked_ship{
                if ship_at_xy==clicked_ship{
                    self.clicked_ship=None;
                    return Some((self.ships[ship_at_xy].1, self.ships[ship_at_xy].2));
                }
            }
        }

        self.clicked_ship=None;
        None
    }
}

impl Drawable for ShipList {
    fn draw(&self, gm: &mut GraphicsManager){
        gm.set_draw_color(OUTER_BORDER_COLOR);
        gm.draw_rect(Rect::new(self.x, self.y, 256, 540));

        for (ship, i) in self.ships.iter().zip(0..){
            gm.draw_text(ship.3, self.x+5+18, self.y+5+20*i);
        }

        gm.set_draw_color(SELECTED_SHIP_COLOR);
        if let Some(x)=self.clicked_ship{
            gm.draw_rect(Rect::new(self.x+5+18, self.y+5+20*(x as i32), 128, 20));
        }

        for &ShipData(_, _, _, _, ref btn) in self.ships.iter(){
            btn.draw(gm);
        }
    }
}