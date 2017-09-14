use diesel;
use diesel::prelude::*;

use shared::globals::Globals as Globals;

use shared::schema::notes::dsl::*;

use models;
use models::note::*;

// TODO return a Result
pub fn index(globals: &Globals) {
    use prettytable::Table;

    // ===

    let notes_list = notes.load::<Note>(&globals.db_conn)
        .expect("Error loading notes"); // TODO copywriting

    info!("{} notes founded", notes_list.len());

    if notes_list.len() > 0 {
        let mut table = Table::new();

        table.add_row(row!["index", "title", "content size after encrypted", "updated at"]);
        for (index, note) in notes_list.iter().enumerate() {
            // TODO dummy timestamp
            table.add_row(row![index, note.title, note.encrypted_content.len(), "2017-01-01 12:10"]);
        }

        table.printstd();
    }
}

// TODO use return a Result<Note, CustomErrorType>
pub fn create(globals: &Globals, creating_note_title: &String) -> Result<usize, diesel::result::Error> {
    #[cfg(not(windows))]
    const EOF: &'static str = "CTRL+D";

    #[cfg(windows)]
    const EOF: &'static str = "CTRL+Z";

    use std::io::{stdin, Read};

    // ===

    println!("Input content for you note \"{}\", (Press {} for EOF when finished)\n", creating_note_title, EOF);
    let mut node_unencrypted_content = String::new();
    stdin().read_to_string(&mut node_unencrypted_content).unwrap();

    models::note::create(&globals.db_conn, &creating_note_title, &node_unencrypted_content)
}
