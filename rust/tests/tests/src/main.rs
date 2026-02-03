use core::error;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::{fs::File, io::BufReader};



async fn read_file(){

    async fn just_read() -> Result<String, io::Error> {
        let mut f: File = File::open("/Users/SamuelBlandon/Desktop/tests/hola.tx")?;
        let mut content = String::new();
        
        let buffer: BufReader<File> = BufReader::new(f);

        buffer.read_to_string(&mut content)?;

        content
    }

    just_read() = match Ok(()){
        
    } 

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
