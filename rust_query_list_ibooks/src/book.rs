
use tabled::{Table, Tabled};

#[derive(Debug)]
#[derive(Tabled)]
pub struct Book {
    pub id: String,
    pub annotations: u32,
    pub title: String,
    pub author: String,
}

pub fn book_list_query() -> String {
    let query = "select 
    ZBKLIBRARYASSET.ZASSETID,
    CASE
        WHEN LENGTH(ZBKLIBRARYASSET.ZTITLE) > 30 THEN
            substr(ZBKLIBRARYASSET.ZTITLE,1,30) || '...'
        ELSE
            ZBKLIBRARYASSET.ZTITLE
        END BookTitle,

    ZBKLIBRARYASSET.ZAUTHOR,    
    count(ae.ZAEANNOTATION.Z_PK)

    from ZBKLIBRARYASSET left join ae.ZAEANNOTATION
        on ae.ZAEANNOTATION.ZANNOTATIONASSETID = ZBKLIBRARYASSET.ZASSETID
    WHERE ae.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT NOT NULL
    GROUP BY ZBKLIBRARYASSET.ZASSETID;";
    String::from(query)
}