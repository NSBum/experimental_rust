
#[derive(Debug)]
pub struct BookInfo {
    pub title: String,
    pub author: String,
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
}
