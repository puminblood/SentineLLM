use chrono::{NaiveDateTime, Utc};
use crate::read_write::read_har_file;
use serde::{Deserialize, Serialize};
use std::error::Error;
use regex::Regex;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
pub struct message{
    pub id: String,
    pub date: String,
    pub content: String,
    pub author: String,
    pub user_agent: String,
}

fn timestamp_to_string(timestamp: f64) -> String {
    // Convertir le timestamp en secondes en NaiveDateTime
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp as i64, (timestamp.fract() * 1e9) as u32)
        .unwrap_or_else(|| Utc::now().naive_utc());

    // Formater en chaîne de caractères
    datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

fn extract_id(data: String) -> io::Result<String>{
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

fn extract_date(data: String) -> io::Result<String>{
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

fn extract_content(data: String) -> io::Result<String>{
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

fn extract_user_agent(data: String) -> io::Result<String>{
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

fn extract_author(data: String) -> io::Result<String>{
/*
Extrait l'auteur de la requête sous format String
*/
    let re = Regex::new(r#""auhtor":{"role":"(.*)""#).unwrap();
    if let Some(captures) = re.captures(&data) {
        Ok(captures.get(1).map(|m| m.as_str().to_string()).unwrap())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Author not found"))
    }
}

pub fn extract_info(path: String) {//-> Result<message, Box<dyn Error>>{
    let data = read_har_file(&path).expect("Read failed !");
    println!("{:?}", extract_author(data))

}