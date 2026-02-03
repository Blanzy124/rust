use core::error;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::{fs::File, io::BufReader};

///Users/SamuelBlandon/Desktop/tests/hola.tx
//C:\Users\samue\OneDrive\Desktop
async fn read_file(){

    async fn just_read() -> Result<String, io::Error> {
        let mut f: File = File::open("C:/Users/samue/OneDrive/Desktop/tests/hola.tx");
        
        let mut content = String::new();
        
        let mut buffer: BufReader<File> = BufReader::new(f);

        buffer.read_to_string(&mut content);
        
        Ok(content);

    }



    println!("Esto es lo que imprime: {:?}", just_read().await);

}

async fn say_hello(){
    println!("Hello, world!");
}

#[tokio::main]
async fn main() {

    let t1 = tokio::spawn(read_file());

    let t2 = tokio::spawn(say_hello());

    tokio::join!(t1, t2);

}
