 mod gym;
 use gym::exercises::exercises;

use crate::gym::exercises::exercises::{Exercises, get_exercises};



#[tokio::main]
async fn main() {

    let mut user_name: String = String::from("SamuelMedinaBlandon");

    print!("{}", exercises::get_exercises(&user_name).await);
}

