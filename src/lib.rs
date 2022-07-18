pub mod schema;
pub mod models;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn register_user<'a>(conn: &PgConnection, did: &'a str) -> User {
    use schema::users;

    let new_user = NewUser{ did: did };
    diesel::insert_into(users::table)
        .values(&new_user)  // a vector of values can be passed here
        .get_result(conn)
        .expect("Could not register user")
}
