use dotenvy::dotenv;
use std::{env};

use sqlx::{MySql, Pool, Error};

use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Serialize)]
struct Exercises_ {
    pub exerciseId: Option<String>,
    pub userName: String,
    pub date: String,
    pub muscleGroup: String,
    pub weight: f32,
    pub rest: f32,
    pub reps: i32,
    pub notes: Option<String>,
}

pub async fn get_exercises(user_name: &String) -> Result<String, Error> {
    dotenv().ok();

    let database_spects: String = env::var("DATABASE_URL").expect("Database spects not found");

    let pool: Pool<MySql> = Pool::connect(&database_spects).await?;

    let responses= sqlx::query!("select bin_to_uuid(exerciseId) as exerciseId, userName, date, muscleGroup, weight, reps, rest, notes from comentsdb.exercises where userName = ?;", user_name).fetch_all(&pool).await?;// the error port doesnt matter
    
    let exercises_: Vec<Exercises_> = responses.into_iter().map(|row| Exercises_ {
        exerciseId: row.exerciseId,
        userName: row.userName,
        date: row.date,
        muscleGroup: row.muscleGroup,
        weight: row.weight,
        rest: row.rest,
        reps: row.reps,
        notes: row.notes,
    }).collect();

    if responses.len() != 0 {
        let json: String = serde_json::to_string(&exercises_).expect("Can not transform to Json");

        return Ok(json)
        
    }
    else {
        return  ;
    }



}