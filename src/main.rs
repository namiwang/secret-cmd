#[macro_use] extern crate clap;
use clap::{App, ArgMatches};

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

mod initializers;
use initializers::globals::Globals as Globals;

mod models;

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
    let db_path = initializers::database::get_db_path();
    let db_conn = initializers::database::establish_connection(&db_path);

    println!("init globals...");

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
    println!("handle notes new");
    println!("{:?}", matches);

    let creating_note_name = matches.value_of("name").unwrap();
    println!("note name: {:?}", creating_note_name);
}

fn handle_notes_list(matches: &ArgMatches, globals: &Globals) {
    println!("handle notes list");
    println!("{:?}", matches);
}
