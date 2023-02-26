

#[derive(Debug)]
pub struct Annotation {
    pub text_selection: String,
    pub note: String,
}

pub fn annotations_query() -> String {
    let query = "select 
    ae.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT,
    CASE 
        WHEN ae.ZAEANNOTATION.ZANNOTATIONNOTE IS NULL THEN ''
        ELSE ae.ZAEANNOTATION.ZANNOTATIONNOTE
    END
from
    ae.ZAEANNOTATION
where 
    ae.ZAEANNOTATION.ZANNOTATIONASSETID = ?1
    AND ae.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT NOT NULL
order by ZPLLOCATIONRANGESTART ASC, ZANNOTATIONCREATIONDATE ASC";
    String::from(query)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_return_query() {
        let query = annotations_query();
        let actual_chunks:Vec<&str> = query.split(" ").collect();
        let actual = actual_chunks[0];
        let expected = String::from("select");
        assert_eq!(actual, expected);
    }
}