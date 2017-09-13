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

infer_schema!("env:SECRET_CMD_DB_PATH");
