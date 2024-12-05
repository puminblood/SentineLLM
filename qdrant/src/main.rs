use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};


fn create_collection() -> Result<(), Box<dyn std::error::Error>>{
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();
    client?
        .create_collection(
            CreateCollectionBuilder::new("my_collection")
                .vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
        );
    println!("Création fini");
    Ok(())
}

async fn _insert_collection() -> Result<(), Box<dyn std::error::Error>>{
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();

    let points = vec![
        PointStruct::new(
            42,                 // Uniqe piont ID
            vec![0.0_f32; 512], // Vector to upsert
            // Attached payload
            [
                ("great", true.into()),
                ("level", 9000.into()),
                ("text", "Hi Qdrant!".into()),
                ("list", vec![1.234f32, 0.815].into()),
            ],
        ),
    ];

    let response = client.expect("REASON")
        .upsert_points(UpsertPointsBuilder::new("my_collection", points))
        .await?;
    dbg!(response);
    Ok(())
}

#[tokio::main]
async fn main(){
    let _ = create_collection();
}