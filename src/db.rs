
use sqlite::{open,Connection};

// Import module init model functions
use crate::core::models::{init_core};
use crate::app::models::{init_users};

pub fn get_conn() -> Connection {
    let path = "./db/main.db3";
    let connection = open(&path).unwrap();

    connection
}

pub fn init_db() -> Connection {

    //let db = Connection::open(&path)?;
    let mut connection = self::get_conn();

    init_core(&mut connection);

    init_users(&mut connection);

    connection
}


// pub fn init_users(connection: &sqlite::Connection) {
//     match connection.execute(
//         "
//         CREATE TABLE users (name TEXT, age INTEGER);
//         INSERT INTO users VALUES ('Alice', 42);
//         INSERT INTO users VALUES ('Bob', 69);
//         ",
//     ) {
//         Ok(_) => (),
//         Err(e) => {
//             println!("Users already exists.. skipping");
//         }
//     }
// }
