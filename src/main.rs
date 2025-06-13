use imageagp::colors; 
use imageagp::texture::image::Image;
use sdl2::render::{Texture,TextureCreator};
use std::path::PathBuf;

const WINDOWTITLE:&str = "AGP";
fn main() {

    let mut theimage = PathBuf::new();
    theimage.push("/home/argument/Downloads/2");
    theimage.set_extension("png");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut window_w = 1920;
    let mut window_h = 1080;

    let mut  window_builder = video_subsystem.window(WINDOWTITLE,window_h, window_w);
   
    let window = window_builder.position_centered().resizable().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    
   let mut imajeri = Image::get_wind_info(sdl_context,&texture_creator,theimage,canvas);
   imajeri.unwrap().normalrunM(); 
    
    
 }
