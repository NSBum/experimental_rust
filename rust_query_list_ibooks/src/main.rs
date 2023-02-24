use rusqlite::{params, Connection, Result};
//use regex::Regex;
use dirs;
use std::path::{PathBuf, Path};
use std::fs::{self, DirEntry};

#[derive(Debug)]
struct Book {
    id: String,
    annotations: u32,
    title: String,
    author: String,
    term: String,
}

// fn find_sqlite_file(path: &Path) -> Option<PathBuf> {
//     let dir_name = path.file_name()?.to_str()?;
//     let file_name = format!("{}.sqlite", dir_name);
//     if let Ok(entries) = fs::read_dir(path) {
//         for entry in entries.flatten() {
//             if let Ok(file_type) = entry.file_type() {
//                 if file_type.is_file() {
//                     if let Some(file_name) = entry.file_name().to_str() {
//                         if file_name == &file_name {
//                             return Some(entry.path());
//                         }
//                     }
//                 } else if file_type.is_dir() {
//                     if let Some(file_path) = find_sqlite_file(&entry.path()) {
//                         return Some(file_path);
//                     }
//                 }
//             }
//         }
//     }
//     None
// }

fn get_last_path_component(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
}

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

fn main() -> Result<()> {
    let mut dir_path = dirs::home_dir().expect("Could not get home directory");
    dir_path.push("Library/Containers/com.apple.iBooksX/Data/Documents/BKLibrary");
    //let last_comp = dir_path.file_name().unwrap().to_string_lossy().into_owned();
    
    //let lc = get_last_path_component(&dir_path);
    //println!("{} is the last comp", lc);

    let path = find_sqlite_file(&dir_path);
    if let Some(path) = find_sqlite_file(&dir_path) {
        println!("{} is the sqlite file", path.display());
    }
    

    Ok(())
}
