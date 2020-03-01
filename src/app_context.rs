
//use sqlite;
use crate::db::{get_conn};
use std::sync::{Mutex};

//Shared mutable state
pub struct AppContext {
    pub requests: Mutex<i64>
}
//Instance state
pub struct AppInstanceContext {
    pub connection: sqlite::Connection,
    //pub statement: Option<sqlite::Statement>
}

impl AppContext {
    pub fn log_request(&self) -> i64 {
        if let Ok(mut requests) = self.requests.lock() {
            *requests += 1;
            let req_count = *requests;
            req_count
        }
        else {
            -1
        }
    }
}

impl AppInstanceContext {
    pub fn new() -> AppInstanceContext {
        AppInstanceContext {
            connection: get_conn(),
            //statement: None
        }
    }
}