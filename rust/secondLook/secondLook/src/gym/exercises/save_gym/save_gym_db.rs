use dotenvy::dotenv;
use std::env;



async fn env(find: &String) -> String{

    dotenv().ok();
    let name = env::var(find).unwrap();
    name
}

struct Response_db{
    pub ok: bool,
    pub message: String,
    pub error_code: i32,
}

impl Response_db {
    fn new(ok: bool, message: String, error_code: i32) -> Self {
        Response_db {
            ok,
            message,
            error_code,
        }
    }
}


pub async fn algo(algo: &String) -> String{

    let name = env(&algo).await;

    return String::from(name);
}

