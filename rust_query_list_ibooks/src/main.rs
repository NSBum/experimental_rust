use rusqlite::{params, Connection, Result};
//use regex::Regex;
use dirs;
use std::path::{PathBuf, Path};
use std::fs::{self};

//#[derive(Debug)]
#[allow(dead_code)]
struct Book {
    id: String,
    annotations: u32,
    title: String,
    author: String,
    term: String,
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

#[allow(unused_variables)]
fn main() -> Result<()> {
    let c = database_connection();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{ibooks_directory_path, get_last_path_component};
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
}