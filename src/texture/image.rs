use sdl2::render::{Canvas,Texture ,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::path::{PathBuf,Path};
use crate::{screen::Screen,colors,filemanagment::{Details,image_metadata}};
use std::fs::{self,Metadata};


pub struct CurrentImage<'a>{
    pub path:PathBuf,
    pub screen:Screen<'a>,
    pub imageindex:usize,
    pub dirimages:Vec<image_metadata>
}


impl<'a> CurrentImage<'a>{
    pub fn get_wind_info(
        sdl_context:Sdl,
        texture_creator:&'a TextureCreator<WindowContext>,
        imgdetails:Details,
        canvas:Canvas<Window>
        ) -> Result<CurrentImage<'a>,String>{
    
            let mut images_data = Vec::new();
            
            for image in imgdetails.otherimages{
                let image_data = image_metadata{
                    path:image.path,
                    data:image.data  
                };

                images_data.push(image_data);
            }
            
            images_data.sort_by();
            
            let mut path:PathBuf = PathBuf::new();
            path.push(&imgdetails.args[1]);
            
            if !path.exists(){
                panic!("not a file");

            }
            let texture = texture_creator.load_texture(&path);
            match texture{
                Ok(b) => b,
                Err(e) => {dbg!(path);
                    panic!("failed to load image");},
            };
            Ok(CurrentImage{
                path:path,
                imageindex:0,
                dirimages:images_data,
            screen: Screen{ 
                    sdl_context,
                    texture_creator,
                    canvas,
                }})
        }
    

   pub fn normalrunM(&mut self){
        

       'main: loop{      

        let texture = self.screen.texture_creator.load_texture(&self.path);
        
        self.screen.canvas.set_draw_color(colors::sdl::BLACK());
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
