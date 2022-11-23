use regex::Regex;


fn main() -> Result<()> {
    let re = Regex::new(r"(?P<vowel>[аеяиыуюэщ])\x{0301}").unwrap();
    let before = String::from("благови́дный");
    let after = re.replace_all(&before, "$vowel");
    println!("{}", after);
}?