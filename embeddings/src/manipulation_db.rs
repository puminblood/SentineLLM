use crate::extract::Embedding;
use serde::{Serialize, Deserialize};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};

pub async fn create_collection() -> Result<(), Box<dyn std::error::Error>>{
/*
    Création de la BDD sur le port 6334
*/
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))  //A voir quoi en faire
    .build();
    let response = client?
        .create_collection(
            CreateCollectionBuilder::new("embeddings")
                .vectors_config(VectorParamsBuilder::new(384, Distance::Cosine)),
        ).await?;
    dbg!(response);
    Ok(())
}

pub async fn insert_collection(embd: &Embedding, id : i64) -> Result<(), Box<dyn std::error::Error>>{
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();

    let points = vec![
        PointStruct::new(
            id as u64,                 //ID
            embd.embedding.clone(), //Vector
            //Payload
            [
                ("label", embd.label.clone().into()),
                ("text", embd.text.clone().into()),
            ],
        ),
    ];

    let response = client.expect("REASON")
        .upsert_points(UpsertPointsBuilder::new("embeddings", points))
        .await?;
    dbg!(response);
    Ok(())
}

pub async fn search_collection() -> Result<(), Box<dyn std::error::Error>>{
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();
    let search_result = client?
    .search_points(
        SearchPointsBuilder::new("embeddings", [11.; 384], 10)  //nom_de_la_BDD, vecteur_recherché, nombre_de_résultats_sortie
            .filter(Filter::all([Condition::matches("text", "Présentation de Naval Group ".to_string())]))  //Filtre de recherche en fonction du contenu du payload
            //.filter(Filter::all([Condition::contains("text", "Naval".to_string())]))  //Autre filtre possible 
            .with_payload(true)  //Récupère également le payload des embbedings retournés
            .params(SearchParamsBuilder::default().exact(true)),  //A voir si utile (impact les performances)
    )
    .await?;
    dbg!(&search_result);
    Ok(())
}


