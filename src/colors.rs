pub enum Colore{
 
    
 
}
 
impl Colore{

}

pub mod sdl{

extern crate sdl2;
use sdl2::pixels::Color;

    pub fn WHITE() -> Color {
        Color::RGB(255,255,255)
    }

    pub fn BLACK() -> Color{
        Color::RGB(0,0,0)
    }
}
