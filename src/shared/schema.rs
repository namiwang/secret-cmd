// TODO maybe should not be in initializers

// TODO
// NOTE currently infer_schema! only supports passing in database_url via env
// so user can't specify/config database path for now
// currently the workaround is let user run command like:
//   DATABASE_PATH=/xxx.db secret notes new
// sorta ref https://github.com/diesel-rs/diesel/issues/689

// so we need a `secret init` command,
// save get_db_path() -> .env file
    // value:
    // let mut database_path = home_dir().unwrap();
    // database_path.push(".secret/data/data.db");

    // // TODO doesnot feel right...
    // database_path.to_str().unwrap().to_string()

// TODO NOTE
// Alternatively to using infer_schema!, you can run diesel print-schema on the command line in your project directory and copy the resulting table! declarations to your schema.rs. You don't need a database to build your application this way, but your schema also won't be updated when you add migrations.

infer_schema!("env:SECRET_CMD_DB_PATH");
