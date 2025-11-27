use core::str;

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

pub fn save_on_db(date: String, muscle_group: String, weight: f64, rest: f64, reps: i64, notes: String) -> Response {
    if date.is_empty() || muscle_group.is_empty() || weight <= 0.0 || rest < 0.0 || reps <= 0 {

        let response: Response =  Response::new(ok: false, String::from("Misiing fields or invalid values"), 400);

    } else if weight >= 1000.0 || rest > 1000.0 || reps >= 1000 {
        return Response{
            ok: false,
            message: String::from("Invalid values for weight, rest or reps"),
            error_code: 422,
        };
    } else {
        return Response{
            ok: true,
            message: String::from("Exercise saved successfully"),
            error_code: 200,
        };
    }
}

pub fn delete_exercise(exercise_id: &str) -> Response {
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
