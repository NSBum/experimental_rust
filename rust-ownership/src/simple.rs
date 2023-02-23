pub fn string_length() {
    let s = String::from("Hello world!");
    let len = calculate_length(&s);

    println!("Length of the string {} is {}", s, len);
}

/*
The code defines a function named calculate_length that takes a reference 
to a String as its argument and returns the length of the String as a usize. 
The function uses the len() method of the String type to calculate the length of the string.

The function takes a reference to a String rather than the String itself 
to avoid taking ownership of the String. This allows the function to read the contents 
of the String without modifying or destroying it. Using a reference instead of taking ownership 
is a common pattern in Rust that helps prevent unintended moves and ensures that the original variable 
remains valid after the function call.

Overall, this code is a simple example of how to define and use 
functions with references in Rust.
 */
fn calculate_length(s: &String) -> usize {
    s.len()
}
