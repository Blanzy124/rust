use serde::{Serialize, Deserialize};
use serde_json;

use crate::gym::exercises::save_gym::save_gym_db::{Response_db, algo};

use super::save_gym_db;
use super::super::exercises::Exercises;

#[derive(Serialize)]
pub struct Response{
    pub ok: bool,
    pub message: String,
    pub error_code: i32,
}

impl Response {
    pub fn new(ok: bool, message: String, error_code: i32) -> Self {
        Response {
            ok,
            message,
            error_code,
        }
    }
}

pub async fn save_on_db(Exercise_: &Exercises) -> String {
    if Exercise_.date.is_empty() || Exercise_.muscle_group.is_empty() || Exercise_.weight <= 0.0 || Exercise_.rest < 0.0 || Exercise_.reps <= 0 {

        let response: Response =  Response::new(false, String::from("Misiing fields or invalid values"), 400);

        let j_: String = serde_json::to_string(&response).expect("Not valid Json");

        j_

    } else if Exercise_.weight >= 1000.0 || Exercise_.rest > 1000.0 || Exercise_.reps >= 1000 {

        let response: Response = Response::new(false, String::from("Not valid numbers"), 402);

        let j_: String = serde_json::to_string(&response).expect("Not valid Json");

        j_
    } else {
        let response: Response_db = save_gym_db::save_exc_db(&Exercise_).await;
        let j_: String = serde_json::to_string(&response).expect("Not valid Json");
        j_

    }
}

pub fn delete_exercise(exercise_id: &String) -> Response {
    if exercise_id == "" || exercise_id.is_empty() {
        return Response{
            ok: false,
            message: String::from("Invalid or missing exercise ID"),
            error_code: 400,
        };
    } else {

        return Response{
            ok: true,
            message: String::from("Exercise deleted successfully"),
            error_code: 200,
        };
    }
}

