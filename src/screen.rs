use sdl2::render::{Canvas,Texture ,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::Sdl;

pub struct Screen<'a>{
    pub sdl_context:Sdl,
    pub texture_creator:&'a TextureCreator<WindowContext>,
    pub canvas:Canvas<Window>
 
}
