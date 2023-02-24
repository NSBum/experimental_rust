
use regex::Regex;

fn split_and_trim(input: &str) -> Vec<String> {
    let re = Regex::new(r"\s+(?:and|&)\s+").unwrap();
    re.split(input)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn remove_degrees(name: &str) -> String {
    let degrees = ["B.A.", "B.S.", "M.A.", "MD", "M.D.", "PhD", "M.S.", "Ph.D.", "Ed.D."];
    let mut new_name = name.to_owned();
    for degree in &degrees {
        new_name = new_name.replace(degree, "");
    }
    new_name.replace(",","").trim().to_string()
}





mod test {
    use super::split_and_trim;
    use super::remove_degrees;

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
    fn test_remove_md_with_periods() {
        assert_eq!(remove_degrees("Marcus Welby, M.D."), "Marcus Welby");
    }

    #[test]
    fn test_remove_md_without_periods() {
        assert_eq!(remove_degrees("Marcus Welby, MD"), "Marcus Welby");
    }

    #[test]
    fn test_remove_phd_with_periods() {
        assert_eq!(remove_degrees("Robert Smith, Ph.D."), "Robert Smith");
    }

    #[test]
    fn test_remove_phd_without_periods() {
        assert_eq!(remove_degrees("Robert Smith, PhD"), "Robert Smith");
    }
}