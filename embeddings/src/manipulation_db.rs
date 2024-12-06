//use crate::extract::Embedding;
//use serde::{Serialize, Deserialize};
use qdrant_client::qdrant::{NamedVectors, PointStruct, UpsertPointsBuilder, Vector, QueryPointsBuilder};
use qdrant_client::{Payload, Qdrant};


pub async fn insert_embd(/*embd: &Embedding*/) -> Result<(), Box<dyn std::error::Error>>{
/*
    Insertion de l'embedding dans la collection embeddings_collection
*/
    let client = Qdrant::from_url("http://localhost:6333").build()?;
    println!("Start insert");
    let points = vec![
        PointStruct::new(1, vec![0.05, 0.61, 0.05, 0.61], [("city", "Berlin".into())]),
        PointStruct::new(2, vec![0.19, 0.81, 0.05, 0.61], [("city", "London".into())]),
        PointStruct::new(3, vec![0.36, 0.55, 0.05, 0.61], [("city", "Moscow".into())]),
    ];
    let response = client
    .upsert_points(UpsertPointsBuilder::new("test_collection", points))
    .await?;
    dbg!(response);

    println!("End insert");
    Ok(())
}

pub async fn read_embd() -> Result<(), Box<dyn std::error::Error>> {
    // URL de l'API pour récupérer les embeddings (exemple fictif)
    let client = Qdrant::from_url("http://localhost:6333").build()?;
    println!("Read start");
    let result = client
        .query(
            QueryPointsBuilder::new("embedding_collection")
                .query(vec![(1, 0.2), (3, 0.1)])
                .limit(10),
                //.using("text"),
        )
        .await?;
    dbg!(result);
    println!("Read finished");
    Ok(())
}

