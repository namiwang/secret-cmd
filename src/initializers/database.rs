use std::env::{home_dir};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// TODO pass in the arg
pub fn get_db_path() -> String {
    let mut database_path = home_dir().unwrap();
    database_path.push(".secret/data/data.db");

    // TODO doesnot feel right...
    database_path.to_str().unwrap().to_string()
}

pub fn establish_connection(db_path: &String) -> SqliteConnection {
    // // TODO kinda ugly here
    // let db_path_buf = get_db_path();
    // let db_path = db_path_buf.to_str().unwrap();

    SqliteConnection::establish(&db_path)
        .expect(&format!("Error connecting to {}", &db_path))
}
