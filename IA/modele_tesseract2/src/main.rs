use anyhow::{Context, Result};
use std::env;
use tesseract::Tesseract;

//Prépare l'image avant OCR : niveaux de gris, enregistrement temporaire.
fn prepare_image_for_ocr(image_path: &str) -> Result<String> {
    let img = image::open(image_path)
        .context("Échec de l'ouverture de l'image")?
        .grayscale();

    let temp_path = "temp_processed.png";
    img.save(temp_path).context("Échec de l'enregistrement de l'image prétraitée")?;

    let extracted_text = run_ocr(temp_path)?;
    
    std::fs::remove_file(temp_path).context("Impossible de supprimer le fichier temporaire")?;
    Ok(extracted_text)
}

//Exécute l'OCR avec Tesseract sur l'image spécifiée.
fn run_ocr(image_path: &str) -> Result<String> {
    let mut ocr = Tesseract::new(None, Some("eng+fra"))
    .context("Échec de l'initialisation de Tesseract")?;

    ocr = ocr.set_variable("tessedit_char_whitelist", "1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyzÀÂÄÇÉÈÊËÎÏÔÖÙÛÜŸàâäçéèêëîïôöùûüÿÆæŒœ0123456789.,!?;:(){}[]<>-_/@#&*+=%€$£ ")
        .context("Impossible de définir la liste des caractères")?;

    ocr = ocr.set_variable("tessedit_pageseg_mode", "3")
        .context("Impossible de définir le mode de segmentation")?;

    ocr = ocr.set_image(image_path).context("Échec du chargement de l'image pour OCR")?;
    let text = ocr.get_text().context("Échec de l'OCR")?;
    Ok(text)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <chemin_image>", args[0]);
        std::process::exit(1);
    }
    
    let image_path = &args[1];
    match prepare_image_for_ocr(image_path) {
        Ok(text) => println!("📝 Texte extrait :\n{}", text),
        Err(e) => eprintln!("❌ Erreur lors de l'OCR : {:?}", e),
    }
    Ok(())
}
