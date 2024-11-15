use tokio;
use aes_gcm::{Aes256Gcm, Key}; 
use aes_gcm::aead::NewAead;
use rand::Rng;
use std::error::Error;
use mongodb::bson::doc;
/*use crate::manipulation_db::{connect_to_mongo, insert_encrypted_har, read_encrypted_har};
use crate::read_write::{read_har_file, write_har_file};
use crate::encryption::{encrypt, decrypt_data};*/
use crate::extraction::extract_info;


mod read_write;
mod manipulation_db;
mod encryption;
mod extraction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    /*let collection = connect_to_mongo().await?;
    let key: [u8; 32] = rand::thread_rng().gen();
    let cipher = Aes256Gcm::new(Key::from_slice(&key));
    let nonce = b"Unique nonce".to_vec();*/

    extract_info("../log.txt".to_string());
    //let data = read_har_file("../log.txt").expect("Read failed !");
    //println!("{:?}", data);
    /*let ciphertext = match encrypt("log.har", &data, &nonce, &cipher){
        Ok(res) => res,
        Err(msg) =>{
            println!("{:?}", msg);
            Err(msg)
        }.expect("Encryption failed")
    };
    let _ = insert_encrypted_har(&collection, &ciphertext).await?;
    
    let filtre = doc! {"name" : "log.har".to_string()};
    let document = read_encrypted_har(&collection, filtre).await?;
    assert_eq!(document.ciphertext, ciphertext.ciphertext);
    let text = decrypt_data(document, &cipher).expect("Decryption failed");
    let _ = write_har_file(text, "logs2.har");*/
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::Client;

    #[tokio::test]
    async fn test_mongodb_connection() {
        // Tente de se connecter à MongoDB
        let client = Client::with_uri_str("mongodb://localhost:27017").await;

        // Vérifie si la connexion est réussie (pas d'erreurs)
        match client {
            Ok(client) => {
                // Si la connexion est réussie, vérifie que la base de données "testDB" est accessible
                let _database = client.database("testDB");
                let dbs = client.list_database_names(None, None).await.unwrap();
                assert!(dbs.contains(&"testDB".to_string()), "La base de données testDB n'existe pas");
            }
            Err(e) => panic!("La connexion à MongoDB a échoué : {:?}", e),
        }
    }
}
