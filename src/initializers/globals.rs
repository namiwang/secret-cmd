// use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct Globals {
    // TODO use &str here is more complicated yet performance friendly, i guess
    pub db_path: String,
    pub db_conn: SqliteConnection,
}
