/*
book-related structs and impls
*/
use std::time::Instant;
use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use crate::{db::DB, error::Result, parse_args::Config};
use walkdir::WalkDir;
static BOOK_FORMATS:[&str;5]=["mobi","txt","epub","azw3","pdf"];
#[derive(Debug)]
pub struct HDDBook {
    pub fname: String,
    pub paths: PathBuf,
    pub usn: u32,
}

impl HDDBook {
    fn new(fname: String, paths: PathBuf, usn: u32) -> Self {
        Self { fname, paths, usn }
    }
}
impl Display for HDDBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "usn: {} ,filename: {}", self.usn, self.fname)
    }
}
#[derive(Debug)]
pub struct Book {
    db: DB,
    hddbook: Option<Vec<HDDBook>>,
    config: Config,
}

impl Book {
    pub fn new(config: Config) -> Result<Self> {
        println!("found db in {}", config.location);
        let db = DB::new(config.location.clone().into())?;
        db.conn.execute_batch(include_str!("create_book.sql"))?;
        Ok(Self {
            db,
            hddbook: None,
            config,
        })
    }
    /// delete all records from db file if it exists and it's not empty
    fn delete_books<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        if path.as_ref().exists() {
            // query records
            if let Some(c)=self.db.count()?  {
                println!("found records {} in db ",c);
                let sql = "delete from books";
                self.db.conn.execute(sql, [])?;
            }
        }
        Ok(())
    }
    ///walk hdd path and 
    /// return book info format  [`HDDBook`] (fname,path,usn) (usn:base usn starts from 1)
    ///
    /// It is the paths or dir (in hdd) which is going to be walked .
    fn get_hdd_books(&self, path: PathBuf) -> Result<Vec<HDDBook>> {
        let mut books = vec![];
        let mut usn = 0;
        for entry in WalkDir::new(path) {
            let entry = entry?.path().to_owned();
            // get file extension,add custome ext if None
            let extension=match entry.extension() {
                Some(e)=>e.to_str().unwrap(),
                None=>"empty"
            };
            if entry.is_file() && BOOK_FORMATS.contains(&extension){
                usn += 1;
                let fname = entry.file_name().unwrap().to_str().unwrap();
                let hddbook = HDDBook::new(fname.to_owned(), entry.clone(), usn);
                books.push(hddbook);
            }
        }
        Ok(books)
    }
    /// walk hdd directorys
    ///
    /// insert info to db
    /// 
    /// to be written info format [`HDDBook`] (fname,path,usn) (usn:base usn starts from 1)
    fn write_book_info(&self, paths: Vec<PathBuf>) -> Result<()> {
        for p in paths {
            let books = self.get_hdd_books(p)?;
            self.db.insert_book_info(books)?;
        }

        Ok(())
    }
    /// logic procedures about regreshing operation
    /// 
    /// only filter files with book format extention,e.g. .mobi,.epub
    fn regresh(&self) -> Result<()> {
        let instant = Instant::now();
        let path = self.config.location.clone();
        self.delete_books(path)?;
        println!("delete books done");
        // paths to be walked in HDD
        let paths = self
            .config
            .refresh
            .as_ref()
            .unwrap()
            .iter()
            .map(|e| Path::new(e).to_owned())
            .collect::<_>();
        self.write_book_info(paths)?;
        let elapse = instant.elapsed().as_secs();
        println!("elapsed time in refreshing hdd {}", elapse);
        Ok(())
    }
    /// print results queried form db.
    fn print(&mut self) {
        let mut n = 0;
        if let Some(books) = self.hddbook.as_mut() {
            for i in books {
                n += 1;
                i.usn = n;
                println!("{}", i);
            }
        } else {
            println!("no book records found in db")
        }
    }
    /// query by words and write bookss info to [`Book`]
    fn query(&mut self) -> Result<()> {
        let words = self.config.query_words.as_ref().unwrap();
        let r = self.db.fetch_all(words)?;
        let mut b = vec![];
        for i in r {
            b.push(i?);
        }
        self.set_hddbook(Some(b));
        Ok(())
    }
    /// copy selected book from hdd to pc storage
    fn copy(&self) -> Result<()> {
        let idx = self
            .config
            .copy
            .as_ref()
            .unwrap()
            .get(0)
            .unwrap()
            .parse::<u32>()?;
        // pc location
        let dst: PathBuf = self.config.copy.as_ref().unwrap().get(1).unwrap().into();
        for i in self.hddbook.as_ref().unwrap() {
            if idx == i.usn {
                let from = &i.paths;
                let to = dst.join(&i.fname);
                let printstr = format!(
                    "copy book {} 
from {}
                
to {:?}
                ",
                    &i.fname,
                    &from.display(),
                    &to
                );
                println!("{}", printstr);
                fs::copy(from, to)?;
                break;
            }
        }

        Ok(())
    }
    pub fn operate_args(&mut self) -> Result<()> {
        if self.config.refresh.is_some() {
            // write book info from hdd to db
            self.regresh()?;
        }
        if self.config.query_words.is_some() {
            // query records from db
            // update BOok by add these records
            self.query()?;
        }
        if self.config.print {
            // just access field books from Book
            // display format usn fname
            self.print();
        }

        if self.config.copy.is_some() {
            // copy selected book by index from results print arg
            self.copy()?;
        }
        Ok(())
    }

    pub fn set_hddbook(&mut self, hddbook: Option<Vec<HDDBook>>) {
        self.hddbook = hddbook;
    }
}
