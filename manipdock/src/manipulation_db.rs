use mongodb::{Client, Collection};
use mongodb::bson::{Bson, doc, Document};
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::extraction::Requete;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedHar{
    pub name: String,
    pub nonce: Bson,
    pub ciphertext: Vec<u8>,
}

pub async fn connect_to_mongo() -> Result<Collection<Requete>, Box<dyn Error>>{
/*
Connexion à MongoDB
Choix de la BDD à utiliser
Choix de la table à manipuler
*/
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("logsDB");
    let collection = database.collection::<Requete>("requests");
    Ok(collection)
}
    
    
pub async fn _insert_encrypted_har(collection: &Collection<EncryptedHar>,encrypted_har : &EncryptedHar) -> Result<(), Box<dyn Error>>{
/*
Insere le document dans la BDD
*/
    collection.insert_one(encrypted_har, None).await?;
    println!("Encrypted HAR document inserted successfully!");
    Ok(())
}

pub async fn _read_encrypted_har(collection: &Collection<EncryptedHar>, filtre : Document) -> Result<EncryptedHar, Box<dyn Error>>{
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


pub async fn insert_request(collection: &Collection<Requete>, request : &Requete) -> Result<(), Box<dyn Error>>{
/*
Insere le document dans la BDD
*/
    collection.insert_one(request, None).await?;
    println!("Request inserted successfully!");
    Ok(())
}

pub async fn read_request(collection: &Collection<Requete>, filtre : Document) -> Result<Requete, Box<dyn Error>>{
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
