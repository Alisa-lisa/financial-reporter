/// reading input data for DB entry from cli

extern crate diesel;
use std::io::{stdin, Read};
use finnicorn::*;

fn main() {
    let connection = establish_connection();

    println!("Submit device id?");
    let mut did = String::new();
    stdin().read_line(&mut did).unwrap();
    let did = &did[..(did.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} (Press {} when finished)\n", did, EOF);
    let user = register_user(&connection, did);
    println!("\nSaved draft {} with id {}", did, user.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
