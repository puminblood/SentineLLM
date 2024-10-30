use aes_gcm::aead::{Aead, Error as AeadError};
use aes_gcm::{Aes256Gcm, Nonce};

pub fn generate_key() -> [u8; 32]{
/*
Génération d'une clé aléatoire pour le chiffrement
*/
    rand::random::<[u8; 32]>()
}

pub fn encrypt_data(data: &[u8], cipher: &Aes256Gcm) -> Result<Vec<u8>, AeadError>{
/*  
Chiffrement des données
*/
    let nonce = Nonce::from_slice(b"unique_nonce");
    let ciphertext = cipher.encrypt(nonce, data)?;
    Ok(ciphertext)
}

pub fn decrypt_data(ciphertext: &[u8], cipher: &Aes256Gcm) -> Result<Vec<u8>, AeadError>{
/*
Déchiffrement des données
*/
    let nonce = Nonce::from_slice(b"unique_nonce"); 
    let decrypted_data = cipher.decrypt(nonce, ciphertext)?;
    Ok(decrypted_data)
}

/* 
fn main() {
    // Initialisation du générateur de nombres aléatoires
    let mut rng = rand::thread_rng();
    let mut data = [0u8; 12];
    rng.fill_bytes(&mut data);

    let key = generate_key();

    let cipher = encrypt_data(&key, &data).expect("Encryption failed");
    let decrypted_data = decrypt_data(&key, &cipher).expect("Decryption failed");

    println!("Data : {:?}\nCipher : {:?}\nDecrypted : {:?}", data, cipher, decrypted_data);
}
*/