fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);

    // Optional additional test cases
    assert_eq!(q1_parser("AB".to_string()), true);     // Exactly two letters
    assert_eq!(q1_parser("A".to_string()), false);     // Too short (< 2)
    assert_eq!(q1_parser("AbC".to_string()), false);   // Lowercase letter
    assert_eq!(q1_parser("A1B".to_string()), false);   // Digit in position 2
    
    println!("All assertions passed successfully!");
}

// Utility method that takes a character and returns true if digit 
// or upper case letter.
fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

// Utility method that takes a character and returns true if it is 
// an upper case letter.
fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn q1_parser(text: String) -> bool {
    // make String into char vector
    let characters_array: Vec<char> = text.chars().collect();

    // The grammar requires at least two uppercase letters
    if characters_array.len() < 2 {
        return false;
    }

    // for each character in the character vector do... 
    // In Rust, .enumerate() yields 0-based indices (0 = 1st char, 1 = 2nd char)
    for (i, character) in characters_array.iter().enumerate() {
        if i < 2 && !is_uppercase_letter(*character) {
            return false;
        } else if i >= 2 && !is_uppercase_or_digit(*character) {
            return false;
        }
    }

    true
}