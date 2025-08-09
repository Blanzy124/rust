use tokio::time::{sleep, Duration};
use tokio::join;

async fn print_something() {
 sleep(Duration::from_secs(3)).await;
 let text: &str  = "Hola desde la funcion";
 println!("{}", text);
 
}

async fn hola() {
 println!("hola hola");
}


#[tokio::main]
async fn main() {

 join!(
  print_something(),
  hola()
 );

    

}
