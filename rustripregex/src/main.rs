use regex::Regex;
use std::env;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
fn get_sentence_alt() -> String {
    // this is an alternate way of doing it, but it
    // requires checking the number of args rather
    // than doing it in a more idiomatic way
    let args: Vec<String> = env::args().collect();
    let mut sentence = "Уверя́ю вас, что подо́бная оши́бка никогда не повтори́тся.".to_string();
    println!("There are {} arguments. First is {}", args.len(), args[0]);
    if args.len() < 2 {
        println!("Since you didn't specify a sentence, using default.");
    } else {
        sentence = args[1].to_string();
    }
    sentence
}

fn get_sentence() -> String {
    let args: Vec<String> = env::args().collect();
    // rather than test args length, or worse, risking
    // panic, we get get an Option<&String> and match
    let param: Option<&String> = args.get(1);
    print_type_of(&param);
    match param {
        Some(param) => param.to_string(),
        None => "Уверя́ю вас, что подо́бная оши́бка никогда не повтори́тся.".to_string(),
    }
}

fn main()  {
    // using named capture group feature $vowel
    // and the \x{} code point nomenclature
    let re = Regex::new(r"(?P<vowel>[аеояиыуюэщ])\x{0301}").unwrap();
    let before = get_sentence();
    let after = re.replace_all(&before, "$vowel");
    println!("{}", after);
}
