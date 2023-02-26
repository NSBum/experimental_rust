use rusqlite::{params, Result};
use clap::{command, Command, arg, Subcommand};

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

/*

 let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("export")
                .about("Exports highlights and notes to Markdown")
                .arg(arg!([NAME])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("export", sub_matches)) => println!(
            "'myapp add' was used, name is: {:?}",
            sub_matches.get_one::<String>("NAME")
        ),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

 */

#[allow(unused_variables)]
fn main() -> Result<()> {
    let matches = command!() // requires `cargo` feature
        .version("0.7")
        .author("Alan Duncan <duncan.alan@me.com>")
        .about("Export your iBooks annotations to markdown")
        .subcommand(
            Command::new("notes")
                .about("Exports annotations to Markdown file")
                .arg(arg!([BOOK_ID]))
                .arg(arg!([PATH])),
        )
        .subcommand(Command::new("list").about("List iBooks with annotations"))
        .get_matches();

    match matches.subcommand() {
        Some(("notes", sub_matches)) => {
            let book_id = sub_matches.get_one::<String>("BOOK_ID")
                .expect("no book id provided");
            let export_path = sub_matches.get_one::<String>("PATH")
                .expect("no path provided");
            println!("'notes' command used, ID is: {:?}", book_id);
        },
        Some(("list", sub_matches)) => {
            println!("book last was used");
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
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