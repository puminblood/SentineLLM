use sqlx::mysql::MySqlPool;
use aes_gcm::aead::KeyInit;
use aes_gcm::{Aes256Gcm, Key};


mod encryption;
mod file;
mod storage;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let source : &str = "/mnt/c/Users/woldo/Downloads/SentineLLM/Code/Bus.pdf";
    
    let source_bytes = match file::read_pdf_to_bytes(source){
        Ok(buf) => buf,
        Err(chaine) => {
            println!("{:?}", chaine);
            Err(chaine)
        }.expect("Read failed")
    };

    let key = encryption::generate_key();
    let key_cipher = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key_cipher);

    let data = encryption::encrypt_data(&source_bytes, &cipher).expect("Encryption failed");

    let database_url = "mysql://dorian:8jtv%jE7L2@localhost/test";
    let pool = MySqlPool::connect(database_url).await?;

    let _ = storage::insert_pdf(&pool, data.clone()).await.expect("Insertion failed");

    let data_read = storage::get_pdf(&pool, 21).await.expect("Get failed");
    
    let decrypted_data = encryption::decrypt_data(&data_read, &cipher).expect("Decryption failed");
    
    let dest : &str = "/mnt/c/Users/woldo/Downloads/SentineLLM/Code/Bus-copy.pdf";
    let _ = file::write_pdf(dest, &decrypted_data);

    Ok(())
}