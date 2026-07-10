use reqwest::Error;
use sqlx::query_as_unchecked;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YarnRequestGetSucces { //This is only use for succes get requests.
    pub ok: bool,
    pub message: String,
    pub data: Data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YarnRequestSucces { //This is use for succesfull request on post, delete, put, patch.
    pub ok: bool,
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  YarnRequestFail {
    pub ok: bool,
    pub message: String,
    pub error_codde: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub yarn: Vec<Yarn>
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Yarn { //Name must be change
    pub lot_number: String,
    pub size: String,
    pub color: String,
    pub available: bool,
    pub content: String,
    pub location: String,
    pub note: String
}

impl Yarn {
    pub fn new (lot_number: String, size: String, color: String, content: String, location: String, note: String) 
    -> Result<Self, String>{
        
        if lot_number.is_empty() || size.is_empty() || !color.is_empty() || location.is_empty(){
            return Err("Lot number, size, color or location are missing".into());
        }

        if !lot_number.is_empty() && !size.is_empty() && !color.is_empty() && !location.is_empty() && !content.is_empty() && !note.is_empty() {
            return Ok(Self { lot_number, size, color, available: true, content, location, note });
        }
        else {
            return Err("Values not acceoted.".into());
        }
        }
}