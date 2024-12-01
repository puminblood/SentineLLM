use crate::extract::extract_text_and_labels;
use extract::Embedding;
use embed_anything::embeddings::local::bert::BertEmbeder;
use crate::manipulation_db::{connect_to_mongo, insert_embedding, read_embedding};
use std::env;
use mongodb::bson::doc;
use tokio;

mod extract;
mod manipulation_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: not enough argument.");
        std::process::exit(1); 
    }
    let pdf_path = &args[1];
    let collection = connect_to_mongo().await?;

    // Extraire le texte et les labels
    let text_and_labels = extract_text_and_labels(pdf_path)?;
    if text_and_labels.is_empty() {
        eprintln!("Error: PDF empty.");
        return Ok(());
    }
    // Initialiser l'embedeur
    let bert_embeder = BertEmbeder::default();

    // Générer les embeddings
    let embeddings: Vec<Embedding> = text_and_labels
        .into_iter()
        .filter(|(text, _)| !text.trim().is_empty())
        .map(|(text, label)| {
            let embedding = bert_embeder
                .embed(&[text.clone()], None)
                .unwrap_or_else(|_| vec![vec![]]);
            Embedding {
                text,
                embedding: embedding[0].clone(),
                label: label.to_string(),
            }
        })
        .collect();

/* 
    // Sauvegarder dans un fichier JSON
    let output_file = "embeddings_labeled_dataset.json";
    let json_data = serde_json::to_string_pretty(&embeddings)?;
    fs::write(output_file, json_data)?;

    println!(
        "Dataset d'embeddings avec labels généré et sauvegardé dans '{}'",
        output_file
    );
*/

    for embedding in embeddings {
        insert_embedding(&collection, &embedding).await?;
    }

    //Test de recherche
    let filtre = doc! {"text" : "Naval Group est un leader mondial dans la conception, la construction et le soutien des ".to_string()};
    let embd = read_embedding(&collection, filtre).await?;
    println!("{:?}", embd);

    
    Ok(())
}