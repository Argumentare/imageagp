use std::env::args_os;
use std::process::Command;
use std::path::{PathBuf,Path};
use glob::{Paths,glob};
use std::ffi::OsString;
use std::fs::{self,Metadata};

pub struct image_metadata{
        pub path:PathBuf,
        pub data:Metadata,
}

pub struct Details{
    
    pub args:Vec<OsString>,
    pub otherimages:Vec<image_metadata>,
    
}


impl Details{
    pub fn get_details()->Details{
        
        let image_extensions:[&str;4] = ["jpg","jpeg","gif","png"];
        let args = args_os().collect::<Vec<_>>();
        
        let mut imagepath:PathBuf = PathBuf::new();
        imagepath.push(&args[1]);
        let currentimagedir = imagepath.parent();
        let mut imagevec = Vec::new();
        for x in 0..image_extensions.len(){

            let buff = glob(format!("{}/*{}",currentimagedir.unwrap().display(),image_extensions[x]).as_str()).unwrap();
            for pat in buff{
            let path = pat.unwrap();
            let metadata = fs::metadata(path.clone());
            let image_data = image_metadata{path:path,data:metadata.unwrap()};
            
            imagevec.push(image_data);
            }
        }
        

        Details{
            args:args,
            otherimages:imagevec,
        }
    }   


}
