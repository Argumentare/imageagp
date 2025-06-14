use sdl2::render::{Canvas,Texture ,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::path::PathBuf;
use crate::screen::Screen;
use crate::colors;


pub struct Image<'a>{
    pub path:PathBuf,
    pub screen:Screen<'a>,
}


impl<'a> Image<'a>{
    pub fn get_wind_info(
        sdl_context:Sdl,
        texture_creator:&'a TextureCreator<WindowContext>,
        currentimage:PathBuf,
        canvas:Canvas<Window>
        ) -> Result<Image<'a>,String>{
        unsafe{
            if !currentimage.exists(){
                panic!("not a file {}",currentimage.display());
            }
            let texture = texture_creator.load_texture(&currentimage);
            match texture{
                Ok(b) => b,
                Err(e) => panic!("failed to load image"),
            };
            Ok(Image{
                path:currentimage,
            screen: Screen{ 
                    sdl_context,
                    texture_creator,
                    canvas,
                }})
        }
    }

   pub fn normalrunM(&mut self){
        
        
       'main: loop{
        let texture = self.screen.texture_creator.load_texture(&self.path);
        
        self.screen.canvas.set_draw_color(colors::sdl::WHITE());
        self.screen.canvas.clear();
        self.screen.canvas.copy(&texture.unwrap(), None, None).unwrap();
        
        for event in self.screen.sdl_context.event_pump().unwrap().poll_iter(){

            match event{
                Event::Quit{..} => break 'main,
                _ => {},
            }
        }

        self.screen.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

    }


   
}
