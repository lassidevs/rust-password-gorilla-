use wasm_bindgen::prelude::*;

pub fn calculate_password_strength(password: &str) -> f64 {
    let length = password.len();

    // points
    let mut length_points = 0.0;
    let mut numcount_points = 0.0;
    let mut uppercase_points = 0.0;
    let mut lowercase_points = 0.0;
    let mut special_chars_points = 0.0;

    // counts
    let mut uppercases = 0.0;
    let mut lowercases = 0.0;
    let mut numcount = 0.0;
    let mut special_chars = 0.0;

    // weights
    let uppercase_weight = 0.9;
    let lowercase_weight = 0.7;
    let numcount_weight = 1.2;
    let length_weight = 2.0;
    let special_chars_weight = 1.5;


    match length {
        0 => 'case_zero: {
            break 'case_zero
        }
        1..=7 => 'block : {
            break 'block
        }
        8..=15 => {
            length_points += 10.0

        } 
        16..=30 => {
            length_points += 20.0

        }
        _ => {
            length_points += 30.0
        }
        
    }
    
   // iterate over password and calculate the number of uppercases, lowercases, special symbols, and numbers 
   // note that numbers are counted as lowercase
    for character in password.chars() {

        if character.is_uppercase() {
            uppercases += 1.0;
        } else {
            lowercases += 1.0;
        }

        if character.is_numeric() {
            numcount += 1.0; 
        } 

        if is_special_symbol(character) {
            special_chars += 1.0;
        }      
    }

    match numcount {
        x if (x - 0.0f64).abs() < std::f64::EPSILON => {
            numcount_points = 0.0;
        }
        x if x >= 1.0 && x <= 3.0 => {
            numcount_points += 10.0;
        }
        x if x >= 4.0 && x <= 6.0 => {
            numcount_points += 20.0;
        }
        _ => {
            numcount_points += 30.0;
        }
    }
    

// For uppercases
match uppercases {
    x if (x - 0.0f64).abs() < std::f64::EPSILON => {
        uppercase_points = 0.0;
    }
    x if x >= 1.0 && x <= 3.0 => {
        uppercase_points += 10.0;
    }
    x if x >= 4.0 && x <= 6.0 => {
        uppercase_points += 20.0;
    }
    _ => {
        uppercase_points += 30.0;
    }
}

// For lowercases
match lowercases {
    x if (x - 0.0f64).abs() < std::f64::EPSILON => {
        lowercase_points = 0.0;
    }
    x if x >= 1.0 && x <= 3.0 => {
        lowercase_points += 10.0;
    }
    x if x >= 4.0 && x <= 6.0 => {
        lowercase_points += 20.0;
    }
    _ => {
        lowercase_points += 30.0;
    }
}

// For special_chars
match special_chars {
    x if (x - 0.0f64).abs() < std::f64::EPSILON => {
        special_chars_points = 0.0;
    }
    x if x >= 1.0 && x <= 3.0 => {
        special_chars_points += 10.0;
    }
    x if x >= 4.0 && x <= 6.0 => {
        special_chars_points += 20.0;
    }
    _ => {
        special_chars_points += 30.0;
    }
}



    println!("uppercases: {uppercases}");
    println!("uppercase_points: {uppercase_points} \n");

    println!("lowercases: {lowercases}");
    println!("lowercase_points: {lowercase_points} \n");

    println!("numcount: {numcount}");
    println!("numcount_points: {numcount_points} \n");

    println!("length: {length}");
    println!("length_points: {length_points} \n");

    println!("special_chars: {special_chars}");
    println!("special_chars_points: {special_chars_points} \n");



// Remember to add customizable weights here before returning the points
// lowercases should have the lowest weight, with uppercases coming in second.
// numericals should be weighted in the middleish
// total length and number of special symbols should be rated highest


let total_points = (uppercase_points * uppercase_weight)
        + (lowercase_points * lowercase_weight)
        + (numcount_points * numcount_weight)
        + (length_points * length_weight)
        + (special_chars_points * special_chars_weight);

    total_points

}


// check if char is a predetermined special symbol
fn is_special_symbol(character: char) -> bool {
    let special_symbols = [
        '!', '@', '#', '$', '%', '^', '&', '*', '-', '+', '=', '(', ')', '[', ']',
        '{', '}', '<', '>', ':', ';', ',', '.', '?', '/', '_', '~', '`', '\'', '"', '\\'
    ];

    special_symbols.contains(&character)
}