use rusqlite::{params, Connection, Result};
//use dirs;
use std::path::{PathBuf};
//use std::fs::{self};
use super::filesys;
//use super::annotations;

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