use pdf2image::{PDF2ImageError, RenderOptionsBuilder, PDF};
use image::ImageFormat;
use pdf2image::image as pdf_image; // Alias pour éviter la confusion
use std::fs;

fn main() -> Result<(), PDF2ImageError> {
    let pdf = PDF::from_file("../image/lipsum.pdf").unwrap();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=8),
        RenderOptionsBuilder::default().pdftocairo(true).build()?,
    )?;

    let img_format = match ImageFormat::Jpeg {
        ImageFormat::Jpeg => pdf_image::ImageFormat::Jpeg,
        ImageFormat::Png => pdf_image::ImageFormat::Png,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Format non supporté").into()),
    };

    fs::create_dir_all("examples/out").unwrap();
    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(format!("examples/out/{}.jpg", i + 1), img_format)?;   
    }
    Ok(())
}


