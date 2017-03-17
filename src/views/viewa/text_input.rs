
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

const UNSELECTED_BKG_COLOR: Color = Color::RGB(75, 75, 75);
const SELECTED_BKG_COLOR: Color = Color::RGB(255, 255, 255);
const TEXT_COLOR: Color = Color::RGB(0, 0, 0);

pub struct TextInput {
    x: i32,
    y: i32,
    selected: bool,
    text: String,
    texts_index: usize,
}

impl TextInput {
    pub fn new(x: i32, y: i32, gm: &mut GraphicsManager) -> TextInput {
        TextInput{x: x, y: y, selected: false, text: String::with_capacity(16), texts_index: gm.get_new_text_index()}
    }

    pub fn get_text(&self) -> String{
        self.text.clone()
    }

    pub fn manage_leftclick(&mut self, x: i32, y: i32){
        self.selected=if x>=self.x && x<=self.x+256 && y>=self.y && y<=self.y+32{
            true
        } else {
            false
        };
    }

    pub fn manage_text_input(&mut self, text: String, gm: &mut GraphicsManager) -> bool{
        if self.selected && self.text.len()<10{
            self.text.push_str(&text);
            gm.set_text_with_bitstream_vera_32(self.texts_index, &self.text, TEXT_COLOR);
            true
        } else {
            false
        }
    }

    pub fn manage_backspace_press(&mut self, gm: &mut GraphicsManager){
        if self.selected{
            self.text.pop();
            gm.set_text_with_bitstream_vera_32(self.texts_index, &self.text, TEXT_COLOR);
        }
    }

    pub fn clear_text(&mut self, gm: &mut GraphicsManager){
        self.text.clear();
        gm.set_text_with_bitstream_vera_32(self.texts_index, &self.text, TEXT_COLOR);
    }
}

impl Drawable for TextInput {
    fn draw(&self, gm: &mut GraphicsManager){
        match self.selected{
            true => gm.set_draw_color(SELECTED_BKG_COLOR),
            false => gm.set_draw_color(UNSELECTED_BKG_COLOR),
        }
        gm.fill_rect(Rect::new(self.x, self.y, 256, 38));
        gm.draw_text(self.texts_index, self.x, self.y);
    }
}
