
pub mod interact;

use std::io;


use reqwest::{self, Response};

use tokio::join;

struct Https 
{
	url: String,
}

impl Https {
	fn https(url: String) -> Https {
		Https { url }
	}

	async fn perform_simple_https(&self) -> Result<String, String>{
		let response: Response = reqwest::get(&self.url).await.map_err(|e: reqwest::Error| e.to_string())?;
	
		let body: String = response.text().await.map_err(|e: reqwest::Error| e.to_string())?;
	
		Ok(body)
	}
} 




#[tokio::main]
async fn main() 
{

	let url: String = "https://blanzynetwork.org:8443".to_string();

	let https = Https::https(url);

	join!();
	match https.perform_simple_https().await {
		
		Ok(body) => println!("Body de la request: \n {}", body),
		Err(e) => eprintln!("Error en la request: {}", e),
	}

	println!("{}", https.url);

}
