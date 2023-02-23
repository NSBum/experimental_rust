use std::fs::File;
use std::io::prelude::*;
//use std::path::PathBuf;
use dirs;

fn main() -> std::io::Result<()> {
    let mut fpath = dirs::home_dir().expect("Could not get home directory");
    fpath.push("Documents");
    fpath.push("this_is_just_a_test.txt");

    // clone fpath bc we want to display later
    // and File::create causes a move
    let fpath_copy = fpath.clone();

    // Create a new File at the specified path
    let mut file = File::create(fpath)?;

    // Write some text to the file
    file.write_all(b"Hello, world!\n")?;

    println!("The path is: {}", fpath_copy.display());

    // Delete the file after writing to it
    /*
    In this example, we use the unwrap_or_else method to call remove_file
    and ignore any errors that might occur. If an error occurs, the closure 
    passed to unwrap_or_else will be called with the error as an argument, 
    but we simply ignore the error and return ().
     */
    std::fs::remove_file(fpath_copy).unwrap_or_else(|_| ());

    Ok(())
}
