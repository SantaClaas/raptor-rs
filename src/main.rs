mod data;
mod raptor;

use rusqlite::{Connection, Result, Statement};

fn main() {
    println!("Hello, world!");

    let connection = Connection::open("database.db").unwrap();

    data::get_data(&connection).expect("oh no");
}
