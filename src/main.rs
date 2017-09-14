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

    let mut globals = init_globals();

    match cli_matches.subcommand_name() {
        Some("auth") => {
            handle_auth(cli_matches.subcommand_matches("auth").unwrap(), &mut globals);
        },
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

fn handle_auth(matches: &ArgMatches, globals: &mut Globals) {
    info!("HANDLING COMMAND: auth");
    info!("with args: {:?}", matches);

    // TODO what if already authed

    // TODO match result and handle error
    if session_controller::create(&globals) {
        println!("Password correct. "); // TODO copywriting
        // TODO generate key and save in globals
        // TODO save os-wise env
        // REF https://stackoverflow.com/questions/496702/can-a-shell-script-set-environment-variables-of-the-calling-shell
        // REF https://stackoverflow.com/questions/16618071/can-i-export-a-variable-to-the-environment-from-a-bash-script-without-sourcing-i
        // TODO
        // and if there isn't any proper and elegant way to do so, we should consider other solutions
        // say, a REPL-like thing
    } else {
        println!("Password incorrect. Aborting.");
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
