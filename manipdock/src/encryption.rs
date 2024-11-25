use aes_gcm::{Aes256Gcm, Nonce}; 
use aes_gcm::aead::Aead;
use mongodb::bson::{Bson, Binary, spec::BinarySubtype};
use crate::manipulation_db::EncryptedHar;


pub fn _bson_binary_from_slice(slice: &Vec<u8>) -> Bson {
    /*
    Convertit un bson en Vec
    */
        Bson::Binary(Binary {
            subtype: BinarySubtype::Generic,
            bytes: slice.to_vec(),
        })
    }
    
pub fn _bson_to_vec(bson: Bson) -> Option<Vec<u8>> {
/*
Convertit un Vec en bson
*/
    if let Bson::Binary(Binary { bytes, .. }) = bson {
        Some(bytes)
    } else {
        None
    }
}

pub fn _encrypt_json(name: &str, data: &[u8], nonce:&Vec<u8> , cipher : &Aes256Gcm) -> Result<EncryptedHar, aes_gcm::aead::Error> {
/*
Chiffre les datas avant de les mettre dans la structure avec son nom et le nonce utilisé
*/
    let nonce_ = Nonce::from_slice(nonce); 
    let ciphertext = cipher.encrypt(nonce_, data)?;
    let result = EncryptedHar {
        name: name.to_string(),
        nonce: _bson_binary_from_slice(nonce),
        ciphertext,
    };
    Ok(result)
}

pub fn _decrypt_data_json(document: EncryptedHar, cipher: &Aes256Gcm) -> Result<Vec<u8>, aes_gcm::aead::Error> {
/*
Déchiffre la struct et renvoie les datas 
*/
    let decrypted_data = cipher.decrypt(Nonce::from_slice(&_bson_to_vec(document.nonce).unwrap()), document.ciphertext.as_slice())?;
    Ok(decrypted_data)
}

pub fn _encrypt_requete(/*id: &String, author: &String, content: &String, date: &String, user_agent: &String*/) {//-> Result<Requete, aes_gcm::aead::Error>{
    println!("");
}