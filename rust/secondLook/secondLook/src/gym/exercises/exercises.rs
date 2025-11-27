pub struct Exercises {
 pub date: String,
 pub muscle_group: String,
 pub weight: f64,
 pub rest: f64,
 pub reps: i64,
 pub notes: String,
}

impl Exercises {
    pub fn new(
        date: String,
        muscle_group: String,
        weight: f64,
        rest: f64,
        reps: i64,
        notes: String,
    ) -> Self {
        Exercises {
            date,
            muscle_group,
            weight,
            rest,
            reps,
            notes,
        }
    }

    pub fn save_exercise(&self) {
        
        println!("Saving exercise: {:?}", self);
    }
    
}
