mod str_reverse;

fn main() {
    let mut string1 = String::from("Hello World!");
    let mut string2 = String::from("Racecar");
    string1 = str_reverse::str_reverse(string1);
    string2 = str_reverse::str_reverse(string2);
    println!("{string1}");
    println!("{string2}");
}
