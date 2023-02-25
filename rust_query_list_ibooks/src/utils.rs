#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use regex::Regex;

fn split_and_trim(input: &str) -> Vec<String> {
    let re = Regex::new(r"\s+(?:and|&)\s+").unwrap();
    re.split(input)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn name_component_count(name: &str) -> usize {
    let name_parts: Vec<&str> = name.split(' ').collect();
    let num_parts = name_parts.len();
    return num_parts;
}

fn extract_last_name(name: &str) -> Option<&str> {
    let degrees = ["Ph.D.", "M.D.", "J.D.", "D.D.S.", "D.V.M.", "D.O.", "D.C."];
    let name_parts: Vec<&str> = name.split(' ').collect();
    let num_parts = name_component_count(name);
    // println!("There are {} parts.", num_parts);
    return match num_parts {
        1 => Some(name_parts[0]),
        2 => Some(name_parts[1]),
        3 => {
            // is this like Dr. Adam Smith?
            if name_parts[0] == "Dr." {
                Some(name_parts[2])
            }
            // if degrees.contains(&name_parts[0]) {
            //     Some(name_parts[2])
            // is it like Cox-Bloom Ph.D., Jim
            else { 
                let maybe_phd = name_parts[1].trim_end_matches(",");
                if degrees.contains(&maybe_phd) {
                    return Some(name_parts[0])
                }
                else {
                    return Some(name_parts[2])
                }
            }
        },
        4 => {
            // is this like Dr. Harold F. Bloom?
            if name_parts[0] == "Dr." {
                return Some(name_parts[3])
            }
            else {
                let maybe_phd = name_parts[1].trim_end_matches(",");
                if degrees.contains(&maybe_phd) {
                    return Some(name_parts[0]);
                }
                None
            }
        }
        _ => Some(name),
    };
}

#[doc = r"Returns formatted authors

Takes a list of authors as a string and returns it formatted
in our preferred way, which is all last names

# Arguments

* `author` - a &str pointing to the raw author list"]
pub fn processed_authors(author: &str) -> Option<String> {
    let authors = split_and_trim(author);
    let auth_count = authors.len();
    if auth_count == 1 {
        let base_auth = extract_last_name(&authors[0]);
        return match base_auth {
            Some(name) => {
                let formatted_auth = format!("{}", name);
                return Some(formatted_auth);
            },
            None => None,
        }
    }
    else if auth_count == 2 {
        let base_auth_1 = extract_last_name(&authors[0]).expect("no auth 1");
        let base_auth_2 = extract_last_name(&authors[1]).expect("no auth 2!");
        let formatted_auth = format!("{} & {}", base_auth_1, base_auth_2);
        return Some(formatted_auth);
    }
    else if auth_count > 2 {
        let mut processed_auth_text = String::from("");
        for author in authors.iter() {
            processed_auth_text.push_str(extract_last_name(author).expect("No auth LN?!"));
            processed_auth_text.push_str(", ");
        }

        return Some(processed_auth_text.trim_end_matches(", ").to_string());
    }
    None
}

mod test {
    use super::*;

    #[test]
    fn test_split_and() {
        let splits = split_and_trim("Jim Fix and John Doe");
        assert_eq!(splits[0], "Jim Fix");
        assert_eq!(splits[1], "John Doe");
    }

    #[test]
    fn test_split_ampersand() {
        let splits = split_and_trim("Heather Horseyface & Brian Bromonger");
        assert_eq!(splits[0], "Heather Horseyface");
        assert_eq!(splits[1], "Brian Bromonger");
    }

    #[test]
    fn test_split_and_only_last_names() {
        let splits = split_and_trim("Fixx and Doe");
        assert_eq!(splits[0], "Fixx");
        assert_eq!(splits[1], "Doe");
    }

    #[test]
    fn test_split_ampersand_only_last_names() {
        let splits = split_and_trim("Horseyface & Bromonger");
        assert_eq!(splits[0], "Horseyface");
        assert_eq!(splits[1], "Bromonger");
    }

    #[test]
    fn test_split_ampersand_extra_white_space() {
        let splits = split_and_trim("Heather Horseyface  &   Brian Bromonger ");
        assert_eq!(splits[0], "Heather Horseyface");
        assert_eq!(splits[1], "Brian Bromonger");
    }

    #[test]
    fn test_split_and_extra_whitespace() {
        let splits = split_and_trim("Jim Fix       and    John Doe ");
        assert_eq!(splits[0], "Jim Fix");
        assert_eq!(splits[1], "John Doe");
    }

    // #[test]
    // fn test_remove_dr_title() {
    //     assert_eq!(remove_suffixes("Dr. John Doe"), "John Doe");
    // }

    #[test]
    fn test_name_with_single_name() {
        assert_eq!(extract_last_name("Smith"), Some("Smith"));
    }
    #[test]
    fn test_name_dual_simple() {
        assert_eq!(extract_last_name("Adam Smith"), Some("Smith"));
    }

    #[test]
    fn test_name_dr_prefix() {
        assert_eq!(extract_last_name("Dr. Ted Bundy"), Some("Bundy"));
    }

    #[test]
    fn test_single_name_components() {
        assert_eq!(name_component_count("Smith"), 1);
    }
    #[test]
    fn test_dual_name_components() {
        assert_eq!(name_component_count("Adam Smith"), 2);
    }
    #[test]
    fn test_three_name_components() {
        assert_eq!(name_component_count("Dr. Nate Zinsser"), 3);
    }

    #[test]
    fn test_dr_with_three_name_parts() {
        assert_eq!(extract_last_name("Dr. Harold L. Bloom"), Some("Bloom"))
    }

    #[test]
    fn test_backwards_phd() {
        assert_eq!(extract_last_name("Chamorro-Premuzic Ph.D., Tomas"), Some("Chamorro-Premuzic"))
    }

    #[test]
    fn test_backwards_phd_with_initial() {
        assert_eq!(extract_last_name("Chamorro-Premuzic Ph.D., Tomas J."), Some("Chamorro-Premuzic"))
    }

    #[test]
    fn test_three_part_name_with_initial() {
        assert_eq!(extract_last_name("John Q. Public"), Some("Public"))
    }

    #[test]
    fn test_fully_processed_single_author() {
        assert_eq!(processed_authors("Adam Smith"), Some("Smith".to_string()));
    }

    #[test]
    fn test_fully_processed_dual_authors_simple() {
        let actual = processed_authors("Harold Bloom & Isaac Stern");
        let expected = Some("Bloom & Stern".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fully_processed_dual_authors_simple_with_initials() {
        let actual = processed_authors("Harold L. Bloom & Isaac R. Stern");
        assert_eq!(actual, Some("Bloom & Stern".to_string()));
    }

    #[test]
    fn test_fully_processed_dual_authors_one_dr() {
        let actual = processed_authors("Dr. Harold Bloom & Isaac Stern");
        let expected = Some("Bloom & Stern".to_string());
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_three_authors() {
        let actual = processed_authors("Jim Bean & Johnny Walker & Hoggish Mann");
        let expected = Some("Bean, Walker, Mann".to_string());
        assert_eq!(actual, expected);
    }
}