use sdl2::render::{Canvas,Texture ,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::Sdl;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::path::{PathBuf,Path};
use crate::{screen::Screen,colors,filemanagment::{Details,image_metadata}};
use std::fs::{self,Metadata};


pub struct CurrentImage<'a>{
    pub screen:Screen<'a>,
    pub imageindex:usize,
    pub imagesdata:Vec<image_metadata>,
}


impl<'a> CurrentImage<'a>{
    pub fn get_wind_info(
        sdl_context:Sdl,
        texture_creator:&'a TextureCreator<WindowContext>,
        imgdetails:Details,
        canvas:Canvas<Window>
        ) -> Result<CurrentImage<'a>,String>{
            
            let mut currentpath:PathBuf = PathBuf::new();
            currentpath.push(&imgdetails.args[1]);
            
            if !currentpath.exists(){
                panic!("not a file");

            }  
    
            let mut images_data = Vec::new();
            for image in imgdetails.otherimages{
                let image_data = image_metadata{
                    path:image.path,
                    data:image.data,  
                    loadable:true,
                };

                images_data.push(image_data);
            }
            
            images_data.sort();
            

            let mut imageindex:usize = 0;
            'finder: for x in 0..images_data.len(){
                if images_data[x].path == currentpath{
                    imageindex = x; 
                    break 'finder;
                }else if x == images_data.len(){
                    panic!("File has been moved or deleted");
                }
            }
            
            for x in 0..images_data.len(){
                
               match  texture_creator.load_texture(&images_data[x].path){
                        Ok(_) => (),
                        Err(_) => {println!("This file is not supported {}",x);
                                   images_data[x].loadable = false},
                }
            }
           
            Ok(CurrentImage{
                imageindex:imageindex,
                imagesdata:images_data,            
                screen: Screen{ 
                    sdl_context,
                    texture_creator,
                    canvas,
                }})
        }
    

   pub fn normalrunM(&mut self){
        

       'main: loop{      

        let texture = self.screen.texture_creator.load_texture(&self.imagesdata[self.imageindex].path);
        
        self.screen.canvas.set_draw_color(colors::sdl::BLACK());
        self.screen.canvas.clear();
        if self.imagesdata[self.imageindex].loadable{
            self.screen.canvas.copy(&texture.unwrap(), None, None).unwrap();
        }else
        {

        }

        for event in self.screen.sdl_context.event_pump().unwrap().poll_iter(){

            match event{
                Event::Quit{..} => break 'main,
                Event::KeyDown{keycode: Some(Keycode::Right),..} =>if self.imageindex < self.imagesdata.len() -1
                                                                    {self.imageindex += 1;},
                Event::KeyDown{keycode: Some(Keycode::Left),..} =>if self.imageindex > 0 
                                                                    {self.imageindex -= 1;},
                _ => {},
            }
        }

        self.screen.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

    }


   
}
