pub struct UserCreation {
    pub User_name: String,
    pub User_password: String,
    pub User_email: String
}

pub struct UserError {
    pub User_error: String,
    pub User_error_code: i32
}

pub async fn create_user_domain(name: &str, password: &str, email: &str) -> Result<UserCreation, UserError> {
    
    fn password_check(passw: &str) -> bool {
        let mut check_box: i32 = 0;
        const ALLOWED_NUMBERS: [char; 10] = ['0','1','2','3','4','5','6','7','8','9'];
        const ALLOWED_SPECIAL_CHARACTERS: [char; 15] = ['!','@','#','$','%','^','&','*','(',')','-','_','+','=','?'];

        for pass in passw.chars() {
            for allowed in ALLOWED_SPECIAL_CHARACTERS.iter() {
                if pass == *allowed {
                    check_box += 1;
                    break;
                }
            }
            for allowed in ALLOWED_NUMBERS.iter() {
                if pass == *allowed {
                    check_box += 1;
                    break;
                }
            }
            if check_box == 2 {
                return true;
            }
            return false;
        }
    return false;
    }
    
    if password_check(password) != true {
        let uError = UserError {
            User_error: String::from("Password must include a number and a special character"),
            User_error_code: 100
        };
        Err(uError)
    }
    fn email_check(ema: &str) -> bool {
        
    }

}