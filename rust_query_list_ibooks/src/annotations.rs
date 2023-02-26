

#[derive(Debug)]
pub struct Annotation {
    pub text_selection: String,
    pub note: String,
}

impl Annotation {
    pub fn markdown(&self) -> String {
        let mut md = format!("> {}", self.text_selection);
        if self.note.chars().count() > 0 {
            md.push_str(&format!("\n\n{}", self.note).to_string());
        }
        md
    }
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

    /// test that we can provide the query that extracts
    /// the annotations from a book
    #[test]
    fn test_can_return_query() {
        let query = annotations_query();
        let actual_chunks:Vec<&str> = query.split(" ").collect();
        let actual = actual_chunks[0];
        let expected = String::from("select");
        assert_eq!(actual, expected);
    }

    #[test]
    /// can we get a single properly-formatted markdown entry?
    fn test_can_export_markdown() {
        let annotation = Annotation {
            text_selection: String::from("something from text"),
            note: String::from("my comment"),
        };
        let actual = annotation.markdown();
        let expected = String::from("> something from text\n\nmy comment");
        assert_eq!(actual, expected);
    }

    /// test that we can export annotation when there's no note
    /// just a highlighted text
    #[test]
    fn test_can_export_markdown_no_note() {
        let annotation = Annotation {
            text_selection: String::from("something from text"),
            note: String::from(""),
        };
        let actual = annotation.markdown();
        let expected = String::from("> something from text");
        assert_eq!(actual, expected);
    }
}