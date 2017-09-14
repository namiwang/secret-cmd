extern crate uuid;

#[macro_use] extern crate log;

#[macro_use] extern crate prettytable;

#[macro_use] extern crate clap;
use clap::{App, ArgMatches};

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

mod shared;
use shared::globals::Globals as Globals;

mod models;

mod controllers;
use controllers::*;

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
    info!("init globals...");

    let db_path = shared::database::get_db_path_from_env().expect("TODO ENV DB PATH NOT EXISTS");

    let db_conn = shared::database::establish_connection(&db_path);

    info!("db_path: {}", db_path);

    Globals {
        db_path,
        db_conn
    }
}

fn handle_notes(matches: &ArgMatches, globals: &Globals) {
    info!("HANDLING COMMAND: notes");
    info!("with args: {:?}", matches);

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
    info!("HANDLEING COMMAND: notes new");
    info!("with args: {:?}", matches);

    let creating_note_title = matches.value_of("title").unwrap().to_string();

    match notes_controller::create(&globals, &creating_note_title) {
        Ok(row_altered) => {
            println!("Note \"{}\" has been saved.", creating_note_title)
        } ,
        Err(err) => {
            panic!("Failed to save the node: {:?}", err)
        }
    }
}

fn handle_notes_list(matches: &ArgMatches, globals: &Globals) {
    info!("HANDLEING COMMAND: notes list");
    info!("with args: {:?}", matches);

    notes_controller::index(globals);
}
