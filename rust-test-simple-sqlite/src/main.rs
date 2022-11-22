use rusqlite::{Connection, Result};


/*

This is a sample project to create a database in Rust.
It is based on the project in the Rust Cookbook but
is modified in a few ways, most importantly because 
sqlite::NO_PARAMS is deprecated.

*/
fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cat_colors (
             id iNTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             color_id INTEGER NOT NULL REFERENCES cat_colors(id)
         )",
        [],
    )?;

    Ok(())
}