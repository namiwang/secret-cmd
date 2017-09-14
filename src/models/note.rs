use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use shared::schema::notes;

use uuid::Uuid;

#[derive(Queryable)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub encrypted_content: String,
}

#[derive(Insertable)]
#[table_name="notes"]
struct NewNote<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub encrypted_content: &'a str,
}

pub fn create(db_conn: &SqliteConnection, note_title: &String, note_unencrypted_content: &String) -> Result<usize, diesel::result::Error> {
    use shared::schema::notes::dsl::*;

    let new_note = NewNote {
        id: &Uuid::new_v4().to_string(),
        title: note_title,
        encrypted_content: note_unencrypted_content, // TODO
    };

    diesel::insert(&new_note).into(notes)
        .execute(db_conn)
}
