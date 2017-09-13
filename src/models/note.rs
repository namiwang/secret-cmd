use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use initializers::schema::notes;

use uuid::Uuid;

#[derive(Queryable)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub encrypted_content: String,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub encrypted_content: &'a str,
}

pub fn create(db_conn: &SqliteConnection, note_title: &String) -> Result<usize, diesel::result::Error> {
    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";

    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";

    use std::io::{stdin, Read};

    use initializers::schema::notes::dsl::*;

    // ===

    println!("Input content for you note \"{}\", (Press {} when finished)\n", note_title, EOF);
    let mut node_unencrypted_content = String::new();
    stdin().read_to_string(&mut node_unencrypted_content).unwrap();

    let new_note = NewNote {
        id: &Uuid::new_v4().to_string(),
        title: note_title,
        encrypted_content: &node_unencrypted_content, // TODO
    };

    diesel::insert(&new_note).into(notes)
        .execute(db_conn)
}
