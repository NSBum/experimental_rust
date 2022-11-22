fn main() {
    let v: Vec<u8> = vec![1,2,3,4,5];

    let sixth: Option<&u8> = v.get(5);
    match sixth {
        Some(sixth) => println!("Exists: {}", sixth),
        None => println!("No sixth element"),
    }
}
