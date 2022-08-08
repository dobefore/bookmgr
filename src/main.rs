use clap::Parser;

mod book;
mod db;
mod parse_args;
use book::Book;
mod error;
use error::Result;
fn main() -> Result<()> {
    //cli argument  parse
    let args = parse_args::Config::parse();
    Book::new(args)?.operate_args()?;
    Ok(())
}
