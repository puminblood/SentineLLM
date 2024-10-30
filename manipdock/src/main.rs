use mongodb::{Client, Collection};
use mongodb::bson::{Bson, Binary, spec::BinarySubtype, doc, Document};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio;
use std::fs::{File, write};
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, NewAead};
use std::io::{self, Read};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
struct EncryptedHar{
    name: String,
    nonce: Bson,
    ciphertext: Vec<u8>,
}

fn bson_binary_from_slice(slice: &Vec<u8>) -> Bson {
/*
Convertit un bson en Vec
*/
    Bson::Binary(Binary {
        subtype: BinarySubtype::Generic,
        bytes: slice.to_vec(),
    })
}

fn bson_to_vec(bson: Bson) -> Option<Vec<u8>> {
/*
Convertit un Vec en bson
*/
    if let Bson::Binary(Binary { bytes, .. }) = bson {
        Some(bytes)
    } else {
        None
    }
}

fn read_har_file(path: &str) -> io::Result<Vec<u8>>{
/*
Lit un fichier et le convertit en Vec
*/
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_har_file(text: Vec<u8>, path: &str) -> io::Result<()>{
/*
Convertit un Vec en un fichier
*/
    write(path, text)
}

async fn connect_to_mongo() -> Result<Collection<EncryptedHar>, Box<dyn Error>>{
/*
Connexion à MongoDB
Choix de la BDD à utiliser
Choix de la table à manipuler
*/
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("testDB");
    let collection = database.collection::<EncryptedHar>("encrypted_har");
    Ok(collection)
}

fn encrypt(name: &str, data: &[u8], nonce:&Vec<u8> , cipher : &Aes256Gcm) -> Result<EncryptedHar, aes_gcm::aead::Error> {
/*
Chiffre les datas avant de les mettre dans la structure avec son nom et le nonce utilisé
*/
    let nonce_ = Nonce::from_slice(nonce); 
    let ciphertext = cipher.encrypt(nonce_, data)?;
    let result = EncryptedHar {
        name: name.to_string(),
        nonce: bson_binary_from_slice(nonce),
        ciphertext,
    };
    Ok(result)
}

fn decrypt_data(document: EncryptedHar, cipher: &Aes256Gcm) -> Result<Vec<u8>, aes_gcm::aead::Error> {
/*
Déchiffre la struct et renvoie les datas 
*/
    let decrypted_data = cipher.decrypt(Nonce::from_slice(&bson_to_vec(document.nonce).unwrap()), document.ciphertext.as_slice())?;
    Ok(decrypted_data)
}

async fn insert_encrypted_har(collection: &Collection<EncryptedHar>,encrypted_har : &EncryptedHar) -> Result<(), Box<dyn Error>>{
/*
Insere le document dans la BDD
*/
    collection.insert_one(encrypted_har, None).await?;
    println!("Encrypted HAR document inserted successfully!");
    Ok(())
}

async fn read_encrypted_har(collection: &Collection<EncryptedHar>, filtre : Document) -> Result<EncryptedHar, Box<dyn Error>>{
/*
Cherche le document selon le filtre appliqué dans la BDD
*/
    let document = match collection.find_one(filtre, None).await?{
        Some(document) => document,
        None => {
            Err("No document found...")
        }.expect("Read failed")
    };
    Ok(document)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let collection = connect_to_mongo().await?;
    let key: [u8; 32] = rand::thread_rng().gen();
    let cipher = Aes256Gcm::new(Key::from_slice(&key));
    let nonce = b"Unique nonce".to_vec();

    let data = read_har_file("PATH/logs.har").expect("Read failed !");
    let ciphertext = match encrypt("log.har", &data, &nonce, &cipher){
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
    let _ = write_har_file(text, "logs2.har");
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
