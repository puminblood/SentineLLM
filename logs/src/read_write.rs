use std::fs::{File, write};
use std::io::{self, Read};

pub fn read_har_file(path: &str) -> io::Result<String>{
    /*
    Lit un fichier et le convertit en Vec
    */
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
    
pub fn _write_har_file(text: Vec<u8>, path: &str) -> io::Result<()>{
/*
Convertit un Vec en un fichier
*/
    write(path, text)
}