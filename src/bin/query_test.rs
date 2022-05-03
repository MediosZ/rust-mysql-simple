use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Db {
    host: String,
    db: String,
}

fn main() -> std::result::Result<(), Error> {
    let url = "mysql://root:wasmtest@127.0.0.1:3306/mysql";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let selected_dbs = conn.query_map("SELECT Host, Db from db;", |(host, db)| Db { host, db })?;
    dbg!(selected_dbs);
    Ok(())
}
