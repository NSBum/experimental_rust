use regex::Regex;

fn main()  {
    // using named capture group feature $vowel
    // and the \x{} code point nomenclature
    let re = Regex::new(r"(?P<vowel>[аеяиыуюэщ])\x{0301}").unwrap();
    let before = String::from("Уверя́ю вас, что подо́бная оши́бка никогда не повтори́тся.");
    let after = re.replace_all(&before, "$vowel");
    println!("{}", after);
}
