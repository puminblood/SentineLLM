use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, ScalarQuantizationBuilder,
    SearchParamsBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};


async fn _create_collection() -> Result<(), Box<dyn std::error::Error>>{
    println!("Start création");
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();
    let response = client?
        .create_collection(
            CreateCollectionBuilder::new("my_collection")
                .vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
        ).await?;
    dbg!(response);
    println!("Création fini");
    Ok(())
}

async fn insert_collection() -> Result<(), Box<dyn std::error::Error>>{
    println!("Start insert");
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();

    let points = vec![
        PointStruct::new(
            43,                 // Uniqe piont ID
            vec![1.0_f32; 512], // Vector to upsert
            // Attached payload
            [
                ("great", true.into()),
                ("level", 9000.into()),
                ("text", "Hello Qdrant!".into()),
                ("list", vec![0.9f32, 1.409].into()),
            ],
        ),
    ];

    let response = client.expect("REASON")
        .upsert_points(UpsertPointsBuilder::new("my_collection", points))
        .await?;
    dbg!(response);
    println!("End insert");
    Ok(())
}

async fn search_collection() -> Result<(), Box<dyn std::error::Error>>{
    println!("Start seraching");
    let client = Qdrant::from_url("http://localhost:6334")
    .api_key(std::env::var("QDRANT_API_KEY"))
    .build();
    let search_result = client?
    .search_points(
        SearchPointsBuilder::new("my_collection", [11.; 512], 10)
            .filter(Filter::all([Condition::matches("text", "Hello Qdrant!".to_string())]))
            .with_payload(true)
            .params(SearchParamsBuilder::default().exact(true)),
    )
    .await?;
    dbg!(&search_result);
    println!("End searching");
    Ok(())
}

#[tokio::main]
async fn main(){
    if let Err(e) = search_collection().await{
        println!("Result :{}", e);
    }
}