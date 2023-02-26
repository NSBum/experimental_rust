use rusqlite::{params, Result};
use clap::Parser;

use tabled::{Table, Tabled};
mod utils;
mod db;
mod filesys;
mod annotations;
mod bookinfo;

#[derive(Debug)]
#[derive(Tabled)]
struct Book {
    id: String,
    annotations: u32,
    title: String,
    author: String,
}

#[allow(unused_variables)]
fn main() -> Result<()> {
    // parse commang line args
    let matches = App::new("ibx")
    .version("0.7")
    .author("Alan Duncan <duncan.alan@me.com>")
    .about("Export your iBooks annotations to markdown")
    .subcommand(SubCommand::with_name("list").about("List your iBooks"))
    .subcommand(
        SubCommand::with_name("notes")
            .about("Export notes and highlights")
            .arg(
                Arg::with_name("FILE")
                    .help("The file path to which notes will be exported")
                    .required(true)
                    .index(1),
            ),
    )
    .get_matches();

match matches.subcommand() {
    ("list", Some(_)) => {
        // handle `ibx list` command
        println!("Listing something...");
    }
    ("notes", Some(matches)) => {
        // handle `ibx notes` command
        let path = matches.value_of("FILE").unwrap();
        println!("Showing notes from file: {}", path);
    }
    _ => unreachable!(),
}
    let c = db::database_connection()?;
    let mut stmt = c.prepare("select 
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
GROUP BY ZBKLIBRARYASSET.ZASSETID;")?;
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
    let book_set: Result<Vec<Book>> = books.collect();
    match book_set {
        Ok(all_books) => {
            let table = Table::new(all_books);
            println!("{}", table);
        }
        Err(error) => {
            println!("Error looking up all books: {:?}", error);
        }
    }
    Ok(())
}