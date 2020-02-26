
//use sqlite;
use std::sync::{Mutex};

pub struct AppContext {
    pub requests: Mutex<i64>
}

pub struct AppInstanceContext {
    pub connection: sqlite::Connection
}
