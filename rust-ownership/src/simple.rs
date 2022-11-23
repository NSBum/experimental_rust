pub fn string_length() {
    let s = String::from("Hello world!");
    let len = calculate_length(&s);

    println!("Length of the string {} is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
