use serde::{Deserialize, Serialize};
use regex::Regex;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct Requete{
    pub id: String,
    pub date: String,
    pub content: String,
    pub author: String,
    pub user_agent: String,
    //pub nonce: Bson,
}

pub fn extract_id(data: &String) -> io::Result<String>{
/*
Extrait l'ID de la requête sous format String
*/
    let re = Regex::new(r#""id":"([a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12})""#).unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "ID not found"))
    }
}

pub fn extract_date(data: &String) -> io::Result<String>{
/*
Extrait la date de la requête sous format String
*/
    let re = Regex::new(r"([0-9]{2} [A-Z]{1}[a-z]{2} [0-9]{4})").unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Date not found"))
    }
}

pub fn extract_content(data: &String) -> io::Result<String>{
/*
Extrait le contenu de la requête sous format String
*/
    let re = Regex::new(r#""parts":\["(.*?)"\]"#).unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Content not found"))
    }
}

pub fn extract_user_agent(data: &String) -> io::Result<String>{
/*
Extrait le user_agent de la requête sous format String
*/
    let re = Regex::new(r"User-Agent: (.*)").unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "User agent not found"))
    }
}

pub fn extract_author(data: &String) -> io::Result<String>{
/*
Extrait l'auteur de la requête sous format String
*/
    let re = Regex::new(r#""role":"([^"]*)""#).unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Author not found"))
    }
}
