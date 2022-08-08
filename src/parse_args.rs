/**
-r,refresh refresh/write book info (logic ops: delete existing db,then create a new db,at last
write new info to db)
-q,query ,query book and ask user either to copy a selected book to PC or to do nothing.

need a required argument dbpath
*/
use clap::Parser;
/// config read from command-line is used in struct Book
#[derive(Parser, Debug)]
#[clap( version, about, long_about = None)]
pub struct Config {
    /// db path/location
    #[clap(short, long, value_parser)]
    pub location: String,
    #[clap(short, long, value_parser)]
    pub query_words: Option<String>,
    /// refreesh database by removing existing db and creating new one and writing new info to it
    /// e.g. -r C D
    #[clap(
        short,
        long,
        multiple_values(true),
        value_name("HDDpath"),
        value_parser
    )]
    pub refresh: Option<Vec<String>>,
    /// print results queried from db if it's true
    #[clap(short, long, action)]
    pub print: bool,
    /// copy a book by an index to a PC storage
    #[clap(short, long,number_of_values(2),multiple_values(true),value_names(&["index", "pclocation"]))]
    pub copy: Option<Vec<String>>,
}
