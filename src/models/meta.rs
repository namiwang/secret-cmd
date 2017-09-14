// use diesel;
// use diesel::prelude::*;
// use diesel::sqlite::SqliteConnection;

use shared::schema::metas;

#[derive(Debug)]
#[derive(Queryable)]
pub struct Meta {
    pub key: String,
    pub value: String,
}
