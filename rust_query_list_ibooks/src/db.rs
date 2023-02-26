use rusqlite::{params, Connection, Result};
//use dirs;
use std::path::{PathBuf};
use crate::book::*;
use crate::annotations::*;
use crate::bookinfo::*;
use super::utils;
//use std::fs::{self};
use super::filesys;

pub fn database_connection() -> Result<Connection> {
    let bk_dir_path = match filesys::ibooks_directory_path("BKLibrary") {
        Some(p) => p,
        None => panic!("No directory path generated"),
    };
    let ae_dir_path = match filesys::ibooks_directory_path("AEAnnotation") {
        Some(p) => p,
        None => panic!("No directory path generated"),
    };
    let bk_path:PathBuf = match filesys::find_sqlite_file(&bk_dir_path) {
        Some(bk_path) => {
            bk_path
        },
        None => panic!("no BKLibrary sqlite file"),
    };
    let ae_path:PathBuf =  match filesys::find_sqlite_file(&ae_dir_path) {
        Some(ae_path) => {
            ae_path
        },
        None => panic!("No AEAnnotation sqlite file"),
    };
    let conn:Connection = Connection::open(bk_path)?;
    let attach_query = format!("ATTACH '{}' as ae", ae_path.display());
    conn.execute(
        &attach_query,
        params![],
    )?;
    
    Ok(conn)
}

pub fn book_list(conn: &Connection) -> Result<Vec<Book>> {
    let query = book_list_query();
    let mut stmt = conn.prepare(&query)?;
    let books = stmt.query_map(params![], |row| {
        // make the authors more presentable
        // before creating the Book struct
        let some_auth:String = row.get(2)?;
        let formatted_author = utils::processed_authors(&some_auth).expect("No author?!");
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: formatted_author,
            annotations: row.get(3)?,
        })
    })?;
    books.collect()
}

pub fn annotations_by_id(id: &str, conn: &Connection) -> Result<Vec<Annotation>> {
    let query = annotations_query();
    let mut stmt = conn.prepare(&query)?;
    // println!("{}",stmt.expanded_sql().expect("***"));
    let rows = stmt.query_map(params![id], |row| {
        Ok(Annotation {
            text_selection: row.get(0)?,
            note: row.get(1)?,
        })
    })?;
    rows.collect()
}

pub fn book_info_by_id(id: &str, conn: &Connection) -> Result<BookInfo> {
    let query = book_info_query();
    let bookinfo = conn.query_row(
        &query,
        [id],
        |row| {
            Ok(BookInfo {
                title: row.get(0)?,
                author: row.get(1)?,
            })
        },
    )?;
    Ok(bookinfo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_annotations() {
        match database_connection() {
            Ok(conn) => {
                let notes = annotations_by_id("03C56CA823C6D9169BD80F0A1960FDE7", &conn);
                assert!(notes.expect("no notes").len() > 0);
            },
            Err(_e) => panic!("No db connection to test"),
        }
    }

    #[test]
    fn test_load_book_info() {
        match database_connection() {
            Ok(conn) => {
                let info = book_info_by_id("03C56CA823C6D9169BD80F0A1960FDE7", &conn);
                assert!(info.expect("No book info").title.chars().count() > 0);
            },
            Err(_e) => panic!("No book info loaded!"),
        }
    }
}