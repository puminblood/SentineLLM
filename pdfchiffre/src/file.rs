use std::fs::{File, write};
use std::io::Read;

pub fn read_pdf_to_bytes(file_path: &str) -> std::io::Result<Vec<u8>>{
/*
Lecture d'un fichier et transformation en vecteur d'octets
*/
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_pdf(file_path: &str, data: &[u8]) -> std::io::Result<()>{
/*
Transformation d'un vecteur d'octet en un fichier
*/
    write(file_path, data)
}

/* 
fn main(){
    let source : &str = "Bus.pdf";
    
    let source_bytes = match read_pdf_to_bytes(source){
        Ok(buf) => buf,
        Err(chaine) => {
            println!("{:?}", chaine);
            Err(chaine)
        }.expect("Read failed")
    };

    let dest : &str = "Bus-copy.pdf";
    let _ = write_pdf(dest, &source_bytes);
}
    */