
#[derive(Debug)]
pub struct BookInfo {
    pub title: String,
    pub author: String,
}

impl BookInfo {
    ///
    /// Returns Markdown output for book-level 
    pub fn markdown(&self) -> String {
        let mut md_out = format!("# Notes from _{}_", self.title);
        md_out.push_str("\n");
        md_out.push_str(&format!("by {}", self.author));
        md_out.push_str("\n\n");
        md_out
    }
}

pub fn book_info_query() -> String {
    let query = "
    select
        ZBKLIBRARYASSET.ZTITLE,
        ZBKLIBRARYASSET.ZAUTHOR
    from ZBKLIBRARYASSET
    where ZBKLIBRARYASSET.ZASSETID = ?1
    ";
    String::from(query)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_return_book_info_query() {
        let query = book_info_query();
        assert!(query.contains("select"));
    }

    #[test]
    fn test_book_info_markdown() {
        let book_info = BookInfo {
            title: "The Fall".to_string(),
            author: "James Dogg".to_string()
        };
        let actual = book_info.markdown();
        let expected = "# Notes from _The Fall_\nby James Dogg\n\n";
        assert_eq!(actual, expected);
    }
}
