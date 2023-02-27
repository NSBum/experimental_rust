use dirs;
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::io::Write;

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
pub fn find_sqlite_file(dir_path: &PathBuf) -> Option<PathBuf> {
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

pub fn save_markdown(md: &str, path: &str) {
    let fpath = PathBuf::from(path);
    // let mut file = File::create(&path);
    match File::create(&path) {
        Ok(mut file) => {
            match file.write_all(md.to_string().as_bytes()) {
                Ok(()) => println!("Markdown file exported to {}", &fpath.display()),
                Err(e) => panic!("Unable to write to path provided - {:?}", e),
            }
        },
        Err(e) => panic!("Unable to write to path provided - {:?}", e),
    }
    
}

#[doc = r#"Returns path to iBooks data directory

Given one of "AEAnnotation" or "BKLibrary" returns the path to
that iBooks data directory

# Arguments

* `name` - the name of the directory of interest
"#]
pub fn ibooks_directory_path(name: &str) -> Option<PathBuf> {
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


#[cfg(test)]
mod test {
    use super::*;

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