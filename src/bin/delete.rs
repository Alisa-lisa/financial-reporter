extern crate diesel;
use finnicorn::*;
use diesel::prelude::*;
use std::io::{stdin};

fn main() {
    use finnicorn::schema::users::dsl::*;

    let connection = establish_connection();
    let mut target_did = String::new();
    stdin().read_line(&mut target_did).unwrap();
    let target_did = &target_did[..(target_did.len() - 1)]; // Drop the newline character

    let num_deleted = diesel::delete(users.filter(did.eq(target_did)))
        .execute(&connection)
        .expect("Could not delete user");

    println!("Deleted {} posts", num_deleted);
}
