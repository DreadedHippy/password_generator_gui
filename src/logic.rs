use rand::{seq::SliceRandom, Rng};
const ALPHABETS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

const SPECIAL_CHARS: [char; 18] = [
    '?', '!', '£', '$', '%', '^', '&', '*', '.', '_', '-', ',', '/', '#', '~', '@', ';', ':'
];

const NUMERIC_CHARS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

pub fn generate_password(password_length: i32, special_chars: i32, numeric_chars: i32) -> String {
    if special_chars + numeric_chars > password_length {
			println!("Password length must be greater than amount of special characters and digits");
			return String::new();
    }
    let mut random_special_chars: Vec<String> = vec![];
    let mut random_numbers: Vec<String> = vec![];
    let mut random_alphabets: Vec<String> = vec![];
    
    for _i in 1..=special_chars {
        random_special_chars.push(SPECIAL_CHARS.choose(&mut rand::thread_rng()).unwrap().to_string());
    }
    
    for _i in 1..=numeric_chars {
        random_numbers.push(NUMERIC_CHARS.choose(&mut rand::thread_rng()).unwrap().to_string());
    }

    for _i in 1..=(password_length - (special_chars+numeric_chars)) {
        random_alphabets.push(generate_alphabet());
    }
    
    
    let mut password: Vec<String> = vec![];
    for _i in 1..=password_length {
        let choice: String = match rand::thread_rng().gen_range(0..=2) {
            0 => {
                if let Some(r) = random_special_chars.pop() {
                    r
                } else if let Some(r) = random_numbers.pop() {
                    r
                } else {
                    random_alphabets.pop().unwrap()
                }
            },
            1 => {
                if let Some(r) = random_numbers.pop() {
                    r
                } else if let Some(r) = random_alphabets.pop() {
                    r
                } else {
                    random_special_chars.pop().unwrap()
                }         
            },
            2 => {
                if let Some(r) = random_alphabets.pop() {
                    r
                } else if let Some(r) = random_special_chars.pop() {
                    r
                } else {
                    random_numbers.pop().unwrap()
                }
            },
            _ => { "This should not have happened".to_string()}
        };

        password.push(choice)        
    }

    let password: String = password.join("");
    return password;


}


fn generate_alphabet() -> String {
    let mut alphabet = ALPHABETS.choose(&mut rand::thread_rng()).unwrap().to_string();

    let is_to_uppercase: bool = match rand::thread_rng().gen_range(0..=1) {
        0 => {true},
        1 => {false},
        _ => {false}
    };

    if is_to_uppercase {
        alphabet = alphabet.to_uppercase();
    }

    return alphabet
}