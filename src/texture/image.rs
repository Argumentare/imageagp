use sdl2::render::{Canvas,Texture ,TextureCreator};
use sdl2::video::{Window,WindowContext};
use sdl2::Sdl;
use sdl2::rect::Rect;
use sdl2::keyboard::{Keycode,Mod};
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::path::{PathBuf,Path};
use std::cmp::Ordering;
use std::env::current_dir;
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
                    scale:1,
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
        let canvas_height = self.screen.canvas.viewport().height();
        let canvas_width = self.screen.canvas.viewport().width();

        self.screen.canvas.set_draw_color(colors::sdl::BLACK());
        self.screen.canvas.clear();
        if self.imagesdata[self.imageindex].loadable{
            
            let scale = self.imagesdata[self.imageindex].scale;
            let texture_query =self.screen.texture_creator.load_texture(&self.imagesdata[self.imageindex].path).unwrap().query();
            
            let mut needed_height = texture_query.height as i32; 
            let mut needed_width = texture_query.width as i32;
    /*        let mut area_coefficient = 0.0;
            let image_area = (texture_query.height as f64*texture_query.width as f64) as i32 ;
            let canvas_area = (canvas_height as f64 *canvas_width as f64) as i32;

            match image_area.cmp(&canvas_area){
                Ordering::Equal => (),
                Ordering::Greater =>{
                                    area_coefficient = canvas_area as f64/image_area as f64;
                                  needed_width = (canvas_area as f64/texture_query.height as f64 *area_coefficient * scale as f64) as i32;    
                                  needed_height = (canvas_area as f64/texture_query.width as f64 *area_coefficient * scale as f64) as i32;    
                                },  
                Ordering::Less =>{area_coefficient = image_area as f64/canvas_area as f64;
                                  needed_width = (image_area as f64/canvas_height as f64 *area_coefficient  * scale as f64) as i32;    
                                  needed_height = (image_area as f64/canvas_width as f64 *area_coefficient * scale as f64) as i32;    
                                  },
            }*/
            'imagesize: for i in 1..canvas_width{
               if texture_query.width > canvas_width || texture_query.height > canvas_height{
                    let height = texture_query.height as i32  / i as i32;
                    let width = texture_query.width  as i32 / i as i32;

                    if height < canvas_height as i32 && width < canvas_width as i32{
                        break 'imagesize;
                    }
                    
                    needed_height = height * scale;
                    needed_width = width * scale;

               }else
               {
                    let height = texture_query.height  as i32* i as i32;
                    let width = texture_query.width  as i32 * i as i32;

                    if height > canvas_height as i32 && width < canvas_width as i32  || width > canvas_width as i32  && height < canvas_height as i32{
                        break 'imagesize;
                    }

                    needed_height = height * scale;
                    needed_width = width  * scale;
               }
            }
            let y = (canvas_height as i32 - needed_height ) /2;
            let x = (canvas_width as i32 - needed_width )/2;
        
            
            self.screen.canvas.copy(&texture.unwrap(), None, Rect::new(x,y,needed_width.try_into().unwrap(),needed_height.try_into().unwrap())).unwrap();
        }
        for event in self.screen.sdl_context.event_pump().unwrap().poll_iter(){

            match event{
                Event::Quit{..} => break 'main,
                Event::KeyDown{keycode: Some(Keycode::Right),..} => if self.imageindex < self.imagesdata.len() -1
                                                                    {self.imagesdata[self.imageindex].scale =1;
                                                                        self.imageindex += 1;},
                Event::KeyDown{keycode: Some(Keycode::Left),..} =>if self.imageindex > 0 
                                                                    {self.imagesdata[self.imageindex].scale = 1;                                                                                                    self.imageindex -= 1;},
                Event::KeyDown{keycode:Some(Keycode::Equals),keymod,..} =>if keymod.contains(Mod::LSHIFTMOD) &&self.imagesdata[self.imageindex].loadable {
                                                                        self.imagesdata[self.imageindex].scale +=1;    
                                                                    },
                Event::KeyDown{keycode: Some(Keycode::MINUS),..} => if self.imagesdata[self.imageindex].loadable && self.imagesdata[self.imageindex].scale > 1{
                                                                      self.imagesdata[self.imageindex].scale -= 1;
                                                                    },
                _ => {},
            }
        }

        self.screen.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

    }


   
}
