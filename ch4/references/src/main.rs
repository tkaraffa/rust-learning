fn main() {
    // & is for referencing something as a pointer
    // referring != owning, referring == borrowing
    // * is for dereferencing something as a pointer
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(&s1);

    println!("The length of '{}' is {}. Also {}", s1, len, s2);
}

fn calculate_length(s: &String) -> (String, usize) {
    let length = s.len();

    (s.to_string(), length)
}
