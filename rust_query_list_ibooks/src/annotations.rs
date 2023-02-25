

#[derive(Debug)]
struct Annotation {
    text_selection: String,
    note: String,
}

pub fn annotations_query() -> String {
    let query = "select 
    a.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT,
    a.ZAEANNOTATION.ZANNOTATIONNOTE
from
    a.ZAEANNOTATION
where 
    a.ZAEANNOTATION.ZANNOTATIONASSETID = ?1
    AND a.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT NOT NULL
order by ZPLLOCATIONRANGESTART ASC, ZANNOTATIONCREATIONDATE ASC";
    String::from(query);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_return_query() {
        let actual_chunks = annotations_query().split(" ").collect();
        let actual = actual_chunks[0];
        let expected = String::from("select");
        assert_eq!(actual, expected);
        
    }
}