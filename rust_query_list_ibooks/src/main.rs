use rusqlite::{Result};
use clap::{command, Command, arg};

use tabled::{Table};
mod utils;
mod db;
mod filesys;
mod annotations;
mod bookinfo;
mod book;


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
            // just to get this book's author and title
            let this_book = db::book_info_by_id(&book_id, &c).expect("no book info!");
            println!("Book title is: {} by {}", this_book.title, this_book.author);
            // now get the annotations
            match db::annotations_by_id(&book_id, &c) {
                Ok(annotations) => {
                    println!("We have annotations");
                    // TODO this is where we will export the annotations
                    let mut md_out = format!("# Notes from _{}_", this_book.title);
                    md_out.push_str("\n");
                    md_out.push_str(&format!("by {}", this_book.author));
                    md_out.push_str("\n");
                    for annotation in annotations {
                        md_out.push_str(&annotation.markdown().to_string());
                        md_out.push_str("\n\n");
                    }
                    println!("Annotations → {}", md_out);
                },
                Err(e) => println!("Error getting annotations {:?}", e),
            }
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