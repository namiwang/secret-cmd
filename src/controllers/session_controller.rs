// use diesel;
use diesel::prelude::*;

use shared::globals::Globals as Globals;

use shared::schema::metas::dsl::*;

use models::meta::*;

// TODO use secure str
// TODO return a Result
pub fn create(globals: &Globals) -> bool {
    use std::io::{stdin, stdout, Read, Write};

    // ===

    print!("Input your password: ");
    stdout().flush();

    let mut input_password = String::new();
    stdin().read_line(&mut input_password).unwrap();
    let input_password = input_password.trim();

    // TODO use secure str and secure compare
    let fetched_metas = metas
        .filter(key.eq("password"))
        .load::<Meta>(&globals.db_conn)
        .expect("Error fetching password in db"); // TODO copywriting
    let encrypted_password = &fetched_metas.first().expect("Error fetching password in db").value;

    info!("fetched encrypted password from db: {}", encrypted_password);

    // TODO use secure str and secure compare
    return encrypted_password == &input_password
}
