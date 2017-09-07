#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};

fn main() {
    let cli_config = load_yaml!("cli.yml");
    let cli_matches = App::from_yaml(cli_config).get_matches();

    match cli_matches.subcommand_name() {
        Some("notes") => {
            handle_notes(cli_matches.subcommand_matches("notes").unwrap());
        },
        _ => {}
    }
}

fn handle_notes(matches: &ArgMatches) {
    println!("handle notes");
    println!("{:?}", matches);

    match matches.subcommand_name() {
        Some("new") => {
            handle_notes_new(matches.subcommand_matches("new").unwrap())
        },
        Some("list") => {
            handle_notes_list(matches.subcommand_matches("list").unwrap())
        },
        _ => {}
    }
}

fn handle_notes_new(matches: &ArgMatches) {
    println!("handle notes new");
    println!("{:?}", matches);

    let creating_note_name = matches.value_of("name").unwrap();
    println!("note name: {:?}", creating_note_name);
}

fn handle_notes_list(matches: &ArgMatches) {
    println!("handle notes list");
    println!("{:?}", matches);
}
