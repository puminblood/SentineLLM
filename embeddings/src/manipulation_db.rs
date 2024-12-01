use mongodb::{Client, Collection};
use mongodb::bson::Document;
use std::error::Error;
use crate::extract::Embedding;


pub async fn connect_to_mongo() -> Result<Collection<Embedding>, Box<dyn Error>>{
/*
Connexion à MongoDB
Choix de la BDD à utiliser
Choix de la table à manipuler
*/
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database = client.database("dataset");
    let collection = database.collection::<Embedding>("embeddings");
    println!("Connexion succeed");
    Ok(collection)
}

pub async fn insert_embedding(collection: &Collection<Embedding>, embed : &Embedding) -> Result<(), Box<dyn Error>>{
/*
Insere le document dans la BDD
*/
    println!("Try to insert...");
    collection.insert_one(embed, None).await?;
    println!("Embedding inserted successfully!");
    Ok(())
}

pub async fn read_embedding(collection: &Collection<Embedding>, filtre : Document) -> Result<Embedding, Box<dyn Error>>{
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
