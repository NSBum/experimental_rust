use rusqlite::{params, Result};
use clap::{command, Command, arg, Subcommand};

use tabled::{Table, Tabled};
mod utils;
mod db;
mod filesys;
mod annotations;
mod bookinfo;
mod book;

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
    let c = db::database_connection()?;
    let matches = command!() // requires `cargo` feature
        .version("0.7")
        .author("Alan Duncan <duncan.alan@me.com>")
        .about("Export your iBooks annotations to Markdown")
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
            println!("book list was used");
            let book_set = db::book_list(&c);
            match book_set {
                Ok(all_books) => {
                    let table = Table::new(all_books);
                    println!("{}", table);
                }
                Err(error) => {
                    println!("Error looking up all books: {:?}", error);
                }
            }
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
    Ok(())
}