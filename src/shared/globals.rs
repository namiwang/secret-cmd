use diesel::sqlite::SqliteConnection;

pub struct Globals {
    pub db_path: String, // TODO use &str here is more complicated yet performance friendly, i guess
    pub db_conn: SqliteConnection,
}
