use rusqlite::{params, Connection, Result};
//use regex::Regex;
use dirs;

#[derive(Debug)]
struct Abbreviation {
    id: i32,
    term: String,
}


fn main() -> Result<()>  {
    let mut db_path = dirs::home_dir().expect("Could not get home directory");
    db_path.push("cli");
    db_path.push("ruabbrevs.sqlite");
    println!("db_path {}", db_path.display());
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare(
        "SELECT * FROM abbrevs;",
    )?;

    let abbrevs = stmt.query_map(params![], |row| {
        Ok(Abbreviation {
            id: row.get(0)?,
            term: row.get(1)?,
        })
    })?;

    let text = "нар.-поэт. самка лебедя; перен., нар.-поэт. то же, что лебёдушка; ласкательное обращение к женщине; техн. приспособление в виде барабана с намотанным на него канатом для подъёма грузов и тяжестей";
    let mut t: String = text.to_string();
    for abbrev in abbrevs {
        // println!("Found term {:?}", abbrev.unwrap().term);
        
        let a = abbrev.unwrap();

        let term = a.term;
        let escaped = format!("_{}_", term);
        println!("{} → {}", term, escaped);
        //let temp: &str = text;//"нар.-поэт. самка";
        t = t.replace(&term, &escaped);
        println!("{}", t);
    }

    Ok(())
}