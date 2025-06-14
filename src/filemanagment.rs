use std::env::args_os;
use std::path::PathBuf;
use std::ffi::OsString;

pub struct Details{
    
    pub args:Vec<OsString>,
}


impl Details{
    pub fn get_details()->Details{
        
        let args = args_os().collect::<Vec<_>>();
        
        Details{
            args:args,
        }
    }   


}
