use tesseract::Tesseract;
use std::path::Path;

fn main() {
    let image_path = "../test1.jpg";

    if !Path::new(image_path).exists() {
        eprintln!("Le fichier image n'existe pas : {}", image_path);
        return;
    }

    match extract_text_from_image(image_path) {
        Ok(text) => {
            println!("Texte extrait de l'image :\n{}", text);
        }
        Err(e) => {
            eprintln!("Erreur lors de l'extraction de texte : {:?}", e);
        }
    }
}

fn extract_text_from_image(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut tess = Tesseract::new(None, Some("fra"))?
    .set_image(image_path)?;
    let text = tess.get_text()?;
    Ok(text)
}
