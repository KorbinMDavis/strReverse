/// Function that reverses a string in Rust

/// Reverses a string!
pub fn str_reverse(string: String) -> String {
    let mut new_string = String::from("");
    //let temp = string;
    for c in string.chars().rev() {
        new_string.push(c);
    }

    return new_string
}