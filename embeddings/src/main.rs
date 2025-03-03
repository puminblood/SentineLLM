//use crate::extract::extract_text_and_labels;
//use extract::Embedding;
//use embed_anything::embeddings::local::bert::BertEmbeder;
use crate::manipulation_db::{insert_embd, read_embd};
//use std::env;
use tokio;

mod extract;
mod manipulation_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let args: Vec<String> = env::args().collect();
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
        .collect();*/
    //let embedding = Embedding{text: "Bonjour".to_string(), embedding: vec![-0.021238107, 0.04356129, -0.034039196, 0.001974411,0.018009825,-0.039301988, -0.042053834], label: "1".to_string()};
    //for embedding in embeddings {
        let _ = insert_embd(/*&embedding*/).await;
    //}

    //Test de recherche
    //let _ = read_embd().await?;

    Ok(())
}

