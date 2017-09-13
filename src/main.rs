extern crate uuid;

#[macro_use] extern crate clap;
use clap::{App, ArgMatches};

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
use diesel::prelude::*;

mod initializers;
use initializers::globals::Globals as Globals;

mod models;
use models::note::*;

fn main() {
    let cli_config = load_yaml!("config/cli.yml");
    let cli_matches = App::from_yaml(cli_config).get_matches();

    let globals = init_globals();

    match cli_matches.subcommand_name() {
        Some("notes") => {
            handle_notes(cli_matches.subcommand_matches("notes").unwrap(), &globals);
        },
        _ => {}
    }
}

fn init_globals() -> Globals {
    println!("init globals...");

    let db_path = initializers::database::get_db_path_from_env().expect("TODO ENV DB PATH NOT EXISTS");

    // init schema
    use initializers::schema;

    let db_conn = initializers::database::establish_connection(&db_path);

    println!("db_path: {}", db_path);

    Globals {
        db_path,
        db_conn
    }
}

fn handle_notes(matches: &ArgMatches, globals: &Globals) {
    println!("handle notes");
    println!("{:?}", matches);

    match matches.subcommand_name() {
        Some("new") => {
            handle_notes_new(matches.subcommand_matches("new").unwrap(), globals)
        },
        Some("list") => {
            handle_notes_list(matches.subcommand_matches("list").unwrap(), globals)
        },
        _ => {}
    }
}

fn handle_notes_new(matches: &ArgMatches, globals: &Globals) {
    println!("*** HANDLEING COMMAND: notes new");
    println!("args: {:?}", matches); // TODO only in debug

    let creating_note_title = matches.value_of("title").unwrap().to_string();

    match models::note::create(&globals.db_conn, &creating_note_title) {
        Ok(row_altered) => {
            println!("Note \"{}\" has been saved.", creating_note_title)
        } ,
        Err(err) => {
            panic!("Failed to save the node: {:?}", err)
        }
    }
}

fn handle_notes_list(matches: &ArgMatches, globals: &Globals) {
    println!("*** HANDLEING COMMAND: notes list");
    println!("args: {:?}", matches); // TODO only in debug

    use initializers::schema::notes::dsl::*;

    let notes_list = notes.load::<Note>(&globals.db_conn)
        .expect("Error loading notes"); // TODO copywriting

    println!("{} notes founded", notes_list.len());

    // TODO table
    // https://github.com/phsym/prettytable-rs
    for note in notes_list {
        println!("{}", note.title);
        println!("----------\n");
        println!("{}", note.encrypted_content.len());
    }
}
