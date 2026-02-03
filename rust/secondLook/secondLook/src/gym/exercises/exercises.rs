use core::error;

use crate::gym::exercises::save_gym::save_gym::Response;

use super::save_gym::save_gym;
use super::save_gym::get_gym;

pub struct Exercises {
 pub date: String,
 pub muscle_group: String,
 pub weight: f64,
 pub rest: f64,
 pub reps: i64,
 pub notes: String,
}

impl Exercises {
    async fn new(
        date: String,
        muscle_group: String,
        weight: f64,
        rest: f64,
        reps: i64,
        notes: Option<String>,
    ) -> Self {
        Exercises {
            date,
            muscle_group,
            weight,
            rest,
            reps,
            notes: notes.unwrap_or("".to_string()),
        }
    }
    pub async fn save_exercise(date: String, muscle_group: String, weight: f64, rest: f64, reps: i64, notes: Option<String>,) -> String {
        let new_exersice: Exercises = Exercises::new(date, muscle_group, weight, rest, reps, notes).await;
        let save: String = save_gym::save_on_db(&new_exersice).await;
        save
    }
    
}

pub async fn get_exercises(user_name: &String) -> String {
 match get_gym::get_exercises(user_name).await {
     Ok(Exercises) => return Exercises,
     Err(error) => return error.to_string(),
 }

}
