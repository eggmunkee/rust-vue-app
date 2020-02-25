
//use sqlite;

#[derive(Clone)]
pub struct AppContext {
    pub requests: i64
}

pub struct AppInstanceContext {
    pub connection: sqlite::Connection
}
