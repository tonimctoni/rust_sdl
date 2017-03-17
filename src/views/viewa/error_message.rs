
use graphics_manager::GraphicsManager;
use graphics_manager::Drawable;
use sdl2::pixels::Color;

const ERROR_TEXT_COLOR: Color = Color::RGB(255, 0, 0);

pub struct ErrorMessage {
    x: i32,
    y: i32,
    texts_index: usize,
    frames_left: usize,
}

impl ErrorMessage {
    pub fn new(x: i32, y: i32, gm: &mut GraphicsManager) -> ErrorMessage{
        ErrorMessage{x: x, y: y, texts_index: gm.get_new_text_index(), frames_left: 0}
    }

    pub fn manage_frame_pass(&mut self){
        if self.frames_left!=0{
            self.frames_left-=1;
        }
    }

    pub fn set_message(&mut self, message: &str, gm: &mut GraphicsManager){
        gm.set_text_with_bitstream_vera_16bd(self.texts_index, message, ERROR_TEXT_COLOR);
        self.frames_left=90;
    }
}

impl Drawable for ErrorMessage {
    fn draw(&self, gm: &mut GraphicsManager){
        if self.frames_left!=0{
            gm.draw_text(self.texts_index, self.x, self.y);
        }
    }
}