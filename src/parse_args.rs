/**
-r,refresh refresh/write book info (logic ops: delete existing db,then create a new db,at last
write new info to db)
-q,query ,query book and ask user either to copy a selected book to PC or to do nothing.

need a required argument dbpath
*/
use clap::{Args, Parser};
/// config read from command-line is used in struct Book
#[derive(Parser, Debug)]
#[clap( version, about, long_about = None)]
pub struct Config {
    /// db path/location
    #[clap(flatten)]
    pub location: Loc,
    /// allow for one wrod or mulpti words. e.g. -q we ; -q "we are"
    #[clap(short, long, value_parser)]
    pub query_words: Option<String>,
    /// refreesh/update database e.g. -r C: D:
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
    /// erase all data form db
    #[clap(short, long, action)]
    pub erase: bool,
}
#[derive(Args, Debug, Clone)]
pub struct Loc {
    dblocation: String,
}

impl Loc {
    pub fn dblocation(&self) -> &str {
        self.dblocation.as_ref()
    }
}
