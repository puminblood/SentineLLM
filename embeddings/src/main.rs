use crate::extract::extract_text_and_labels;
use extract::Embedding;
use embed_anything::embeddings::local::bert::BertEmbeder;
use crate::manipulation_db::{insert_collection, search_collection, create_collection};
use std::env;
use tokio;

mod extract;
mod manipulation_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*if let Err(e) = create_collection().await{   //Création de la BDD à appeller une première fois seul puis à laisser commenté
        println!("Result :{}", e);
    }// */

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: not enough argument.");
        std::process::exit(1); 
    }
    let pdf_path = &args[1];

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
    
    let mut i = 1;
    for embed in embeddings{
        if let Err(e) = insert_collection(&embed, i).await{  
            println!("Insert Result :{}", e);
        }
        i += 1;
    }

    //Test de recherche
    if let Err(e) = search_collection().await{  
        println!("Search Result :{}", e);
    }// */
    Ok(())
}

