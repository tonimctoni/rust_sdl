
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use graphics_manager::BUTTON_DIMS;


pub struct Button {
    x: i32,
    y: i32,
    button_index: usize,
    being_pressed: bool,
    // was_pressed: bool,
}

impl Button {
    pub fn new(x: i32, y: i32, button_index: usize) -> Button {
        Button{x: x, y: y, being_pressed: false, button_index: button_index}
    }

    pub fn set_pos(&mut self, x: i32, y: i32){
        self.x=x;
        self.y=y;
    }

    pub fn manage_leftclick(&mut self, x: i32, y: i32){
        self.being_pressed=if x>=self.x && x<=self.x+(BUTTON_DIMS[self.button_index].0 as i32) && y>=self.y && y<=self.y+(BUTTON_DIMS[self.button_index].1 as i32){
            true
        } else {
            false
        };
    }

    pub fn manage_unleftclick(&mut self, x: i32, y: i32) -> bool{
        if self. being_pressed && x>=self.x && x<=self.x+(BUTTON_DIMS[self.button_index].0 as i32) && y>=self.y && y<=self.y+(BUTTON_DIMS[self.button_index].1 as i32){
            // self.was_pressed=true;
            self. being_pressed=false;
            true
        }else{
            self. being_pressed=false;
            false
        }

        // self. being_pressed=false;
    }

    // pub fn was_pressed(&mut self) -> bool{
    //     let aux=self.was_pressed;
    //     self.was_pressed=false;
    //     aux
    // }
}

impl Drawable for Button {
    fn draw(&self, gm: &mut GraphicsManager){
        match self.being_pressed{
            false => gm.draw_unpressed_button(self.button_index, self.x,self.y),
            true => gm.draw_pressed_button(self.button_index, self.x,self.y),
        }
    }
}