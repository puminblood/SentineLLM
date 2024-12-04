//use crate::extract::Embedding;
//use serde::{Serialize, Deserialize};
use qdrant_client::qdrant::{NamedVectors, PointStruct, UpsertPointsBuilder, Vector, QueryPointsBuilder};
use qdrant_client::{Payload, Qdrant};


pub async fn insert_embd(/*embd: &Embedding*/) -> Result<(), Box<dyn std::error::Error>>{
/*
    Insertion de l'embedding dans la collection embeddings_collection
*/
    let client = Qdrant::from_url("http://localhost:6333").build()?;

    let points = vec![PointStruct::new(
        1,
        NamedVectors::default().add_vector(
            "text",
            Vector::new_sparse(vec![1, 3, 5, 7], vec![0.1, 0.2, 0.3, 0.4]),
        ),
        Payload::new(),
    )];
    client
    .upsert_points(UpsertPointsBuilder::new("{embedding_collection}", points))
    .await?;

    Ok(())
}

pub async fn read_embd() -> Result<(), Box<dyn std::error::Error>> {
    // URL de l'API pour récupérer les embeddings (exemple fictif)
    let client = Qdrant::from_url("http://localhost:6333").build()?;
    println!("Read start");
    let result = client
        .query(
            QueryPointsBuilder::new("embedding_collection")
                .query(vec![(1, 0.2), (3, 0.1), (5, 0.9), (7, 0.7)])
                .limit(10),
                //.using("text"),
        )
        .await?;
    dbg!(result);
    println!("Read finished");
    Ok(())
}

