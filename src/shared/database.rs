use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn get_db_path_from_env() -> Result<String, env::VarError> {
    env::var("SECRET_CMD_DB_PATH")
}

pub fn establish_connection(db_path: &String) -> SqliteConnection {
    SqliteConnection::establish(&db_path)
        .expect(&format!("Error connecting to {}", &db_path))
}
