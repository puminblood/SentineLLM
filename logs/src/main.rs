use extraction::Requete;
use tokio;
use std::env;
use std::error::Error;
use mongodb::bson::doc;
use crate::manipulation_db::{connect_to_mongo, insert_request, read_request};
use crate::read_write::read_har_file;
use crate::extraction::{extract_id, extract_author, extract_content, extract_date, extract_user_agent};


mod read_write;
mod manipulation_db;
mod encryption;
mod extraction;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: not enough argument.");
        std::process::exit(1); 
    }
    let path = &args[1];

    let collection = connect_to_mongo().await?;
    let data = read_har_file(&path).expect("Read failed !");

    let request = Requete{
        id: extract_id(&data).unwrap(),
        author: extract_author(&data).unwrap(),
        content: extract_content(&data).unwrap(),
        date: extract_date(&data).unwrap(),
        user_agent: extract_user_agent(&data).unwrap(),
    };

    let _ = insert_request(&collection, &request).await?;
    let filtre = doc! {"author" : "user".to_string()};

    let request_read = read_request(&collection, filtre).await?;
    println!("{:?}", request_read);
    
    Ok(())
}

