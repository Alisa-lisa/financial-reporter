extern crate diesel;
use diesel::prelude::*;
use finnicorn::models::*;
use finnicorn::*;

fn main() {
    use finnicorn::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users // .filter(did.eq("OlololNr"))
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);  // how to show multiple values if not USer.ToString()
        println!("{}", user.did);
        println!("----------\n");
    }
}
