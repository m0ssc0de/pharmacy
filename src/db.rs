pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::*;
use schema::medicines;
use schema::medicines::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_medicines() -> Vec<Medicine> {
    let connection = establish_connection();
    medicines
        .filter(published.eq(true))
        .limit(4)
        .load::<Medicine>(&connection)
        .expect("Error loading Medicine")
}

use uuid::Uuid;
pub fn create_med(t: &str, p: i32) -> String {
    let connection = establish_connection();

    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    let new_med = NewMedicine {
        id: &uuid,
        title: t,
        price: p,
        descr: "",
    };

    diesel::insert_into(medicines::table)
        .values(&new_med)
        .execute(&connection)
        .expect("Error saving new post");

    uuid
}

pub fn publish_med(key: String) -> usize {
    let connection = establish_connection();

    diesel::update(medicines.find(key))
        .set(published.eq(true))
        .execute(&connection)
        .expect("Unable to find post")
}
