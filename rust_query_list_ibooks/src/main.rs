use rusqlite::{params, Connection, Result};
//use regex::Regex;
use dirs;
use std::path::{PathBuf, Path};
use std::fs::{self};
use tabled::{Table, Tabled};
mod utils;

#[derive(Debug)]
#[derive(Tabled)]
struct Book {
    id: String,
    annotations: u32,
    title: String,
    author: String,
}

/**
Returns the last path component of a path

# Arguments

* `path` - the file path from which to extract the last component


*/
fn get_last_path_component(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
}

/**
Returns an Option<PathBuf> for the main sqlite file
inside an iBooks directory. Each data directory has a 
single sqlite database. This function finds that database 
and returns a path to it.

# Arguments

* `dir_path` - the directory path of an iBooks data directory

*/
fn find_sqlite_file(dir_path: &PathBuf) -> Option<PathBuf> {
    let prefix = get_last_path_component(dir_path);
    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.starts_with(&prefix) && file_name.ends_with(".sqlite") {
                        let file_path = entry.path();
                        if let Ok(metadata) = fs::metadata(&file_path) {
                            if metadata.is_file() {
                                return Some(file_path);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Returns path to iBooks data directory
/// 
/// Given one of "AEAnnotation" or "BKLibrary" returns the path to
/// that iBooks data directory
/// 
/// # Arguments
/// 
/// * `name` - the name of the directory of interest
/// 
fn ibooks_directory_path(name: &str) -> Option<PathBuf> {
    match name {
        "AEAnnotation" | "BKLibrary" => {
            let mut dir_path = dirs::home_dir().expect("Could not get home directory");
            dir_path.push("Library/Containers/com.apple.iBooksX/Data/Documents/");
            dir_path.push(name);
            Some(dir_path)
        },
        _ => None
    }
    
}

fn database_connection() -> Result<Connection> {
    let bk_dir_path = match ibooks_directory_path("BKLibrary") {
        Some(p) => p,
        None => panic!("No directory path generated"),
    };
    let ae_dir_path = match ibooks_directory_path("AEAnnotation") {
        Some(p) => p,
        None => panic!("No directory path generated"),
    };
    let bk_path:PathBuf = match find_sqlite_file(&bk_dir_path) {
        Some(bk_path) => {
            println!("The bk_path TEST is {}", bk_path.display());
            bk_path
        },
        None => panic!("no BKLibrary sqlite file"),
    };
    let ae_path:PathBuf =  match find_sqlite_file(&ae_dir_path) {
        Some(ae_path) => {
            println!("The ae_path TEST is {}", ae_path.display());
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

// fn print_all_books(conn: &Connection) {
// //     let query = "select 
// //         ZBKLIBRARYASSET.ZASSETID,
// //         ZBKLIBRARYASSET.ZTITLE,
// //         ZBKLIBRARYASSET.ZAUTHOR,    
// //         count(a.ZAEANNOTATION.Z_PK)
// // from ZBKLIBRARYASSET left join a.ZAEANNOTATION
// //     on a.ZAEANNOTATION.ZANNOTATIONASSETID = ZBKLIBRARYASSET.ZASSETID
// // WHERE a.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT NOT NULL
// // GROUP BY ZBKLIBRARYASSET.ZASSETID;";
    
// }

fn extract_last_name(input: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?i)(?:(?:^|\s)(?:Dr\.|Mr\.|Mrs\.|Ms\.|Miss\.|Sir\.|Prof\.|Rev\.|Hon\.|Ph\.D\.|MD\.|D\.D\.S\.))?\s*([A-Za-z']+),?").unwrap();
    let caps = re.captures(input)?;
    let mut last_name = caps[1].to_string();

    // Remove any titles that may appear after the last name
    let title_re = regex::Regex::new(r"(?i)\b(?:Dr\.|Mr\.|Mrs\.|Ms\.|Miss\.|Sir\.|Prof\.|Rev\.|Hon\.|Ph\.D\.|MD\.|D\.D\.S\.)\b").unwrap();
    last_name = title_re.replace_all(&last_name, "").trim().to_string();

    Some(last_name)
}

fn format_name(input: &str) -> Option<String> {
    let last_name = extract_last_name(input)?;
    Some(format!("({})", last_name))
}



#[allow(unused_variables)]
fn main() -> Result<()> {
    let c = database_connection()?;
    let mut stmt = c.prepare("select 
    ZBKLIBRARYASSET.ZASSETID,
    ZBKLIBRARYASSET.ZTITLE,
    ZBKLIBRARYASSET.ZAUTHOR,    
    count(ae.ZAEANNOTATION.Z_PK)
from ZBKLIBRARYASSET left join ae.ZAEANNOTATION
on ae.ZAEANNOTATION.ZANNOTATIONASSETID = ZBKLIBRARYASSET.ZASSETID
WHERE ae.ZAEANNOTATION.ZANNOTATIONSELECTEDTEXT NOT NULL
GROUP BY ZBKLIBRARYASSET.ZASSETID;")?;
    let books = stmt.query_map(params![], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            annotations: row.get(3)?,
        })
    })?;
    /* 
    for book in books {
        println!("Found book {:?}", book);
    }
    let all = books.values().cloned().collect();
    */
    let book_set: Result<Vec<Book>> = books.collect();
    match book_set {
        Ok(all_books) => {
            //println!("Found books: {:?}", all_books);
            let table = Table::new(all_books);
            println!("{}", table);
        }
        Err(error) => {
            println!("Error looking up all books: {:?}", error);
        }
    }
    
    match format_name("Dr. John Doe") {
        Some(name) => println!("{}", name),
        None => println!("Nothing"),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{ibooks_directory_path, get_last_path_component};
    use crate::{format_name};
    use crate::Path;

    #[test]
    fn test_ibooks_dir_not_valid() {
        assert_eq!(ibooks_directory_path("DogFish"), None)
    }
    #[test]
    fn test_ibooks_dir_valid_aeannotation() {
        assert_ne!(ibooks_directory_path("AEAnnotation"), None)
    }
    #[test]
    fn test_ibooks_dir_valid_bklibrary() {
        assert_ne!(ibooks_directory_path("BKLibrary"), None)
    }
    #[test]
    fn test_last_path_comp() {
        assert_eq!(get_last_path_component(Path::new("/Users/dog/fish")), "fish")
    }

    #[test]
    fn test_name_dr_pre_period() {
        match format_name("Dr. John Doe") {
            Some(name) => {
                assert_eq!(name, "(Doe)");
            },
            None => assert!(0 > 1),
        }
    }
}