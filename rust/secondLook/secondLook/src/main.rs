 mod gym;
 use gym::exercises::exercises;

use crate::gym::exercises::exercises::Exercises;


fn main() {
    let exercise1: Exercises = Exercises::new(
        String::from("2024-06-15"),
        String::from("Chest"),
        80.0,
        90.0,
        10,
        String::from("Felt strong today"),
    );  

    println!("Exercise on {}: {} - {}kg x {} reps (Rest: {}s) Notes: {}", 
        exercise1.date, 
        exercise1.muscle_group, 
        exercise1.weight, 
        exercise1.reps, 
        exercise1.rest, 
        exercise1.notes);
}

