use pdf_extract::extract_text;
use embed_anything::embeddings::local::bert::BertEmbeder;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Embedding {
    text: String,
    embedding: Vec<f32>,
    label: i32, // 1 pour "sensible", 0 pour "non sensible"
}

fn extract_text_and_labels(pdf_path: &str,) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
    let content = extract_text(pdf_path)?;
    let mut results = Vec::new();
    let mut label = 0;

    for line in content.lines() {
        if line.contains("[SENSIBLE]") {
            label = 1;
        } else if line.contains("[END]") {
            label = 0;
        } else {
            results.push((line.to_string(), label));
        }
    }

    Ok(results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdf_path = "../test_embedding.pdf";

    // Extraire le texte et les labels
    let text_and_labels = extract_text_and_labels(pdf_path)?;
    if text_and_labels.is_empty() {
        eprintln!("Erreur : le PDF est vide ou ne contient pas de texte.");
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
                label,
            }
        })
        .collect();

    // Sauvegarder dans un fichier JSON
    let output_file = "embeddings_labeled_dataset.json";
    let json_data = serde_json::to_string_pretty(&embeddings)?;
    fs::write(output_file, json_data)?;

    println!(
        "Dataset d'embeddings avec labels généré et sauvegardé dans '{}'",
        output_file
    );

    Ok(())
}