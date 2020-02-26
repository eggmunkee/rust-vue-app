
use sqlite::{open,Connection};

// Import module init model functions
use crate::core::models::{init_core};
use crate::app::models::{init_app_models};

pub fn get_conn() -> Connection {
    let path = "./db/main.db3";
    let connection = open(&path).unwrap();

    connection
}

pub fn init_db() -> Connection {

    //let db = Connection::open(&path)?;
    let mut connection = self::get_conn();

    init_core(&mut connection);

    init_app_models(&mut connection);

    connection
}
