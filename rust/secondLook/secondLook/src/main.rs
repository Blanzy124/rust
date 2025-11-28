 mod gym;
 use gym::exercises::exercises;

use crate::gym::exercises::exercises::{Exercises, get_exercises};



#[tokio::main]
async fn main() {
    print!("{}", exercises::get_exercises().await)
}

