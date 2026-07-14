pub struct UserCreation {
    pub user_name: String,
    pub user_password: String,
    pub user_email: String
}

pub struct UserError {
    pub user_error: String,
    pub user_error_code: i32
}

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

fn email_check(email: &str) -> bool {
    let mut check_box: i32 = 0;

    const ALLOWED_SPECIAL_CHARACTERS_FOR_EMAIL: [char; 3] = ['.','@','-'];
    const ARROBA: char = '@';
    const PUNTO: char = '.'; 
    for ema in email.chars() {
        if ema == ARROBA || ema == PUNTO {
            check_box += 1;
        }
    }
}


pub async fn create_user_domain(name: &str, password: &str, email: &str) -> Result<UserCreation, UserError> {
    
    if password_check(password) != true {
        let u_error: UserError = UserError {
            user_error: String::from("Password must include a number and a special character"),
            user_error_code: 100
        };
        Err(u_error)
    }

}