// query records by %%,模糊查询
// write records to db,
// Error: Sqlite(SqliteFailure(Error { code: Unknown, extended_code: 1 }, Some("near \")\": syntax error")))
// fix : no coma , in define table field.
use std::{path::PathBuf, result};

use rusqlite::{params, types::FromSql, Connection, OptionalExtension};

use crate::{book::HDDBook, error::Result};
#[derive(Debug)]
pub struct DB {
    pub conn: Connection,
}

impl DB {
    /// create databse and table if not exist
    pub fn new(path: PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }
    /// return last usn (usn of latest record),0 if no records
    fn last_usn(&self) -> Result<u32> {
        let sql = "SELECT usn FROM books ORDER BY usn DESC";
        if let Some(usn) = self.fetchone(sql, None)? {
            Ok(usn)
        } else {
            Ok(0)
        }
    }
    pub fn fetch_all(
        &self,
        params: &String,
    ) -> Result<Vec<result::Result<HDDBook, rusqlite::Error>>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT fname, paths, usn FROM books where fname like '%{}%'",
            params
        ))?;
        let person_iter = stmt
            .query_map([], |row| {
                Ok(HDDBook {
                    fname: row.get(0)?,
                    paths: row.get::<_, String>(1)?.into(),
                    usn: row.get(2)?,
                })
            })?
            .collect::<Vec<_>>();
        Ok(person_iter)
    }
    /// count amounts of records
    pub fn count(&self) -> Result<Option<u32>> {
        let sql = "SELECT count() FROM books";
        Ok(self.fetchone(sql, None)?)
    }
    /// insert results to db
    ///
    /// 1. query  last_usn in existing db
    ///
    /// 2.insert usn by last usn plus current usn
    pub fn insert_book_info(&self, books: Vec<HDDBook>) -> Result<()> {
        let ls = self.last_usn()?;
        let sql = "INSERT OR REPLACE INTO books VALUES (?,?,?)";
        for e in books {
            let usn = ls + e.usn;
            self.conn
                .execute(sql, params![e.fname, format!("{}", e.paths.display()), usn])?;
        }
        Ok(())
    }
    /// query and getch one record frpm db,return None if there is emtpy record in it.
    ///
    /// params use:e.g.
    /// if we want to query field `usn` and pass this field through params
    /// ```
    /// let sql = "SELECT ? FROM books ORDER BY usn DESC";
    /// fetchone(sql,Some("usn"));
    /// ```
    fn fetchone<T: FromSql>(
        &self,
        sql: &str,
        param: Option<&String>,
    ) -> result::Result<Option<T>, rusqlite::Error> {
        if let Some(p) = param {
            self.conn.query_row(sql, [p], |row| row.get(0)).optional()
        } else {
            self.conn.query_row(sql, [], |row| row.get(0)).optional()
        }
    }
}

#[test]
fn test_db() {
    let db = DB::new("t.db".into()).unwrap();
    db.conn
        .execute_batch(include_str!("create_book.sql"))
        .unwrap();
}
