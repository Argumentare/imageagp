use imageagp::colors;
use imageagp::filemanagment::Details;
use imageagp::texture::image::CurrentImage;
use sdl2::render::{Texture,TextureCreator};
use std::path::PathBuf;
use std::env::{self,args_os,current_dir,current_exe};

const WINDOWTITLE:&str = "AGP";
fn main() {
    
    let mut window_w = 1920;
    let mut window_h = 1080;
    
    let mut theimage = PathBuf::new();
    let imgdetails = Details::get_details();    


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut  window_builder = video_subsystem.window(WINDOWTITLE,window_w, window_h);
       
    let window = window_builder.position_centered().resizable().build().unwrap();
    
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();
    
   let mut imajeri = CurrentImage::get_wind_info(sdl_context,&texture_creator,imgdetails,canvas);
   imajeri.unwrap().normalrunM(); 
    
    
 }
