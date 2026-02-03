use dotenvy::dotenv;
use std::env;

use serde::{Serialize, Deserialize};

use super::super::exercises::Exercises;

async fn env(find: &String) -> String{
    dotenv().ok();
    let name = env::var(find).unwrap();
    name
}

#[derive(Serialize)]
pub struct Response_db{
    pub ok: bool,
    pub message: String,
    pub error_code: Option<i32>,
}

impl Response_db {
    fn new(ok: bool, message: String, error_code: Option<i32>) -> Self {
        Response_db {
            ok,
            message,
            error_code,
        }
    }
}

pub async fn save_exc_db(exercises_: &Exercises) -> Response_db {

    let response_db: Response_db = Response_db::new(true, String::from("Exercise succesfully saved"), None);

    response_db

}


pub async fn algo(algo: &String) -> String{

    let name = env(&algo).await;

    return String::from(name);
}

