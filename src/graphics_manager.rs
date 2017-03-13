use sdl2::render::Renderer;
use sdl2::render::Texture;
use std::path::Path;
use sdl2_image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
// use sdl2::render::TextureQuery;

type EightShipUnits = [Texture;8];
type FiveShips = [Texture;5];
type FiveShipIcons = [Texture;5];
pub struct GraphicsManager {
    renderer: Renderer<'static>,
    ship_units: Box<EightShipUnits>,
    ships: Box<FiveShips>,
    ship_icons: Box<FiveShipIcons>,
}

const SHIP_UNIT_WIDTH: u32 = 28;
const SHIP_UNIT_HEIGHT: u32 = 28;

const SHIP_WIDTH: u32 = 400;
const SHIP_HEIGHT: u32 = 400;

const SHIP_ICON_WIDTH: u32 = 80;
const SHIP_ICON_HEIGHT: u32 = 80;

impl GraphicsManager {
    pub fn init(renderer: Renderer<'static>) -> GraphicsManager{
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
            renderer: renderer,
        }
    }

    pub fn draw_ship_unit(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ship_units[index], None/*Some(Rect::new(0,0,SHIP_UNIT_WIDTH,SHIP_UNIT_HEIGHT))*/, Some(Rect::new(x,y,SHIP_UNIT_WIDTH,SHIP_UNIT_HEIGHT))).unwrap();
    }

    pub fn draw_ship(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ships[index], None/*Some(Rect::new(0,0,SHIP_UNIT_WIDTH,SHIP_UNIT_HEIGHT))*/, Some(Rect::new(x,y,SHIP_WIDTH,SHIP_HEIGHT))).unwrap();
    }

    pub fn draw_ship_icon(&mut self, index: usize, x: i32, y: i32){
        self.renderer.copy(&self.ship_icons[index], None/*Some(Rect::new(0,0,SHIP_UNIT_WIDTH,SHIP_UNIT_HEIGHT))*/, Some(Rect::new(x,y,SHIP_ICON_WIDTH,SHIP_ICON_HEIGHT))).unwrap();
    }

    // pub fn load_ship_units(&mut self){
    //     if self.ship_units.is_none(){
    //         self.ship_units=Some(box [
    //             self.renderer.load_texture(Path::new("assets/ship_units/cargo_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/laser_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/missile_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/plasma_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/scanner_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/shield_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/engine_unit.png")).unwrap(),
    //             self.renderer.load_texture(Path::new("assets/ship_units/cloak_unit.png")).unwrap(),
    //             ]);
    //     }
    // }

    // pub fn load_image(&mut self){
    //     self.some_image=Some(box self.renderer.load_texture(Path::new("assets/brick.png")).unwrap());
    // }

    // pub fn draw_some_image(&mut self, rect: Rect){
    //     if let Some(ref mut x)=self.some_image{
    //         let TextureQuery{width, height, ..}=x.query();
    //         let this_rect=Rect::new(0,0,width,height);
    //         self.renderer.copy(x, Some(this_rect), Some(rect)).unwrap();
    //     } else {
    //         panic!("panic!");
    //     }
    // }
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
