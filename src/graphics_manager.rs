use sdl2::render::Renderer;
use sdl2::render::Texture;
use std::path::Path;
use sdl2_image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2_ttf::Sdl2TtfContext;
use sdl2_ttf::Font;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureQuery;

type EightShipUnits = [Texture;8];
type FiveShips = [Texture;5];
type FiveShipIcons = [Texture;5];
type ThreeButtons = [(Texture,Texture);3];
pub struct GraphicsManager<'a> {
    renderer: Renderer<'static>,
    ship_units: Box<EightShipUnits>,
    ships: Box<FiveShips>,
    ship_icons: Box<FiveShipIcons>,
    buttons: Box<ThreeButtons>,
    bitstream_vera_32:Font<'a>,
    // bitstream_vera_16:Font<'a>,
    bitstream_vera_16bd:Font<'a>,
    texts: Vec<Texture>,
}

const SHIP_UNIT_WIDTH: u32 = 28;
const SHIP_UNIT_HEIGHT: u32 = 28;

const SHIP_WIDTH: u32 = 400;
const SHIP_HEIGHT: u32 = 400;

const SHIP_ICON_WIDTH: u32 = 80;
const SHIP_ICON_HEIGHT: u32 = 80;

// const BUTTON_WIDTH: u32 = 32;
// const BUTTON_HEIGHT: u32 = 32;

pub static BUTTON_DIMS: [(u32,u32);3] = [(32,32), (64,28), (16,16)];

impl<'a> GraphicsManager<'a> {
    pub fn init(renderer: Renderer<'static>, ttf_context: &'a Sdl2TtfContext) -> GraphicsManager<'a>{
        GraphicsManager{
            ship_units: box [
                renderer.load_texture(Path::new("assets/ship_units/cargo_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/laser_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/missile_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/plasma_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/scanner_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/shield_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/engine_unit.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_units/cloak_unit.png")).unwrap(),
                ],
            ships: box [
                renderer.load_texture(Path::new("assets/ships/ship00.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ships/ship01.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ships/ship02.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ships/ship03.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ships/ship04.png")).unwrap(),
            ],
            ship_icons: box [
                renderer.load_texture(Path::new("assets/ship_icons/ship_icon00.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_icons/ship_icon01.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_icons/ship_icon02.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_icons/ship_icon03.png")).unwrap(),
                renderer.load_texture(Path::new("assets/ship_icons/ship_icon04.png")).unwrap(),
            ],
            buttons : box[
                (renderer.load_texture(Path::new("assets/buttons/ok_blue.png")).unwrap(), renderer.load_texture(Path::new("assets/buttons/ok_red.png")).unwrap()),
                (renderer.load_texture(Path::new("assets/buttons/clear_blue.png")).unwrap(), renderer.load_texture(Path::new("assets/buttons/clear_red.png")).unwrap()),
                (renderer.load_texture(Path::new("assets/buttons/x_blue.png")).unwrap(), renderer.load_texture(Path::new("assets/buttons/x_red.png")).unwrap()),
            ],
            bitstream_vera_32: ttf_context.load_font(Path::new("assets/ttf/bitstream_vera_sans/Vera.ttf"), 32).unwrap(),
            // bitstream_vera_16: ttf_context.load_font(Path::new("assets/ttf/bitstream_vera_sans/Vera.ttf"), 16).unwrap(),
            bitstream_vera_16bd: ttf_context.load_font(Path::new("assets/ttf/bitstream_vera_sans/VeraBd.ttf"), 16).unwrap(),
            texts: Vec::with_capacity(64),
            renderer: renderer,
        }
    }

    pub fn empty_texts(&mut self){
        self.texts.clear();
    }

    pub fn get_new_text_index(&mut self) -> usize{
        self.texts.push(self.renderer.create_texture_static(PixelFormatEnum::RGB332, 1, 1).unwrap());
        if self.texts.len()>=64{panic!("Possible memory leak: FIX IT!!");}
        self.texts.len()-1
    }

    pub fn set_text_with_bitstream_vera_32(&mut self, index: usize, text: &str, color: Color){
        if text.len()>0{
            self.texts[index]=self.renderer.create_texture_from_surface(self.bitstream_vera_32.render(text).blended(color).unwrap()).unwrap();
        } else {
            self.texts[index]=self.renderer.create_texture_static(PixelFormatEnum::RGB332, 1, 1).unwrap();
        }
    }

    // pub fn set_text_with_bitstream_vera_16(&mut self, index: usize, text: &str, color: Color){
    //     if text.len()>0{
    //         self.texts[index]=self.renderer.create_texture_from_surface(self.bitstream_vera_16.render(text).blended(color).unwrap()).unwrap();
    //     } else {
    //         self.texts[index]=self.renderer.create_texture_static(PixelFormatEnum::RGB332, 1, 1).unwrap();
    //     }
    // }

    pub fn set_text_with_bitstream_vera_16bd(&mut self, index: usize, text: &str, color: Color){
        if text.len()>0{
            self.texts[index]=self.renderer.create_texture_from_surface(self.bitstream_vera_16bd.render(text).blended(color).unwrap()).unwrap();
        } else {
            self.texts[index]=self.renderer.create_texture_static(PixelFormatEnum::RGB332, 1, 1).unwrap();
        }
    }

    pub fn draw_text(&mut self, index: usize, x: i32, y: i32){
        let TextureQuery{width, height, ..}=self.texts[index].query();
        self.renderer.copy(&self.texts[index], None, Some(Rect::new(x,y,width,height))).unwrap();
    }

    pub fn draw_unpressed_button(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.buttons[index].0, None, Some(Rect::new(x,y,BUTTON_DIMS[index].0,BUTTON_DIMS[index].1))).unwrap();
    }

    pub fn draw_pressed_button(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.buttons[index].1, None, Some(Rect::new(x,y,BUTTON_DIMS[index].0,BUTTON_DIMS[index].1))).unwrap();
    }

    pub fn draw_ship_unit(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ship_units[index], None, Some(Rect::new(x,y,SHIP_UNIT_WIDTH,SHIP_UNIT_HEIGHT))).unwrap();
    }

    pub fn draw_ship(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ships[index], None, Some(Rect::new(x,y,SHIP_WIDTH,SHIP_HEIGHT))).unwrap();
    }

    pub fn draw_ship_icon(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ship_icons[index], None, Some(Rect::new(x,y,SHIP_ICON_WIDTH,SHIP_ICON_HEIGHT))).unwrap();
    }

    // pub fn rounded_rectangle(&mut self, x1: i16, y1: i16, x2: i16, y2: i16, rad: i16, color: Color){
    //     self.renderer.rounded_rectangle(x1,y1,x2,y2,rad,color).unwrap();
    // }
    pub fn draw_rect(&mut self, rect: Rect){
        self.renderer.draw_rect(rect).unwrap();
    }
    pub fn fill_rect(&mut self, rect: Rect){
        self.renderer.fill_rect(rect).unwrap();
    }
    pub fn set_draw_color(&mut self, color: Color){
        self.renderer.set_draw_color(color);
    }
    pub fn clear(&mut self){
        self.renderer.clear();
    }
    pub fn present(&mut self){
        self.renderer.present();
    }
}

pub trait Drawable {
    fn draw(&self, &mut GraphicsManager);
}