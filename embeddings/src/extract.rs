use pdf_extract::extract_text;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Embedding {
    pub text: String,
    pub embedding: Vec<f32>,
    pub label: i32, // 1 pour "sensible", 0 pour "non sensible"
}

pub fn extract_text_and_labels(pdf_path: &str,) -> Result<Vec<(String, i32)>, Box<dyn std::error::Error>> {
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