
use sqlite::{Type,Error};

// define columns of types
//use crate::{create_table, columns, col_text, col_in, insert_row, val_text, val_int};
const CORE_INIT_TABLE: &str = "core_app_init";

pub fn init_core(connection: &sqlite::Connection) {
    let cmd = create_table!(CORE_INIT_TABLE,columns!(col_text!("table_name"),col_int!("init")));
    println!("Init app_init table cmd: {}", &cmd);
    match connection.execute( &cmd ) {
        Ok(_) => {  },
        Err(e) => {
            println!("Error trying to add app_init. \n Error: {}\n Cmd: {}", e, &cmd);
        }
    }
    
}

pub fn add_init_table(connection: &sqlite::Connection, table_name: &str) -> Result<(),()> {
    let cmd = insert_row!(CORE_INIT_TABLE, format!("'{}',1", &table_name) );
    println!("INIT USERS app_init: {}", &cmd);
    match connection.execute( &cmd ) {
        Ok(_) =>  { println!("App__init Users [created]."); Ok(()) },
        Err(e) => {
            println!("Error adding Users init value..\n. Error: {}\n Cmd: {}", e, &cmd);
            Err(())
        }
    }
}

pub fn get_type_name<'a>(statement: &'a sqlite::Statement, index: usize) -> &'a str {
    match &statement.kind(index) {
        Type::String => {
            "string"
        },
        Type::Integer => {
            "integer"
        },
        Type::Float => {
            "float"
        },
        Type::Binary => {
            "binary"
        },
        Type::Null => {
            "null"
        }
    }    
}
pub fn get_string(statement: &sqlite::Statement, index: usize) -> Result<String, String> {
    match &statement.kind(index) {
        Type::String => match statement.read::<String>(index) {
            Ok(string_value) => Ok(string_value),
            Err(_) => Err(String::from(format!("Column {} not string", &index)))
        },
        _ => {
            Err(String::from(format!("Column {} not string", &index)))
        }
    }    
}

pub fn get_int(statement: &sqlite::Statement, index: usize) -> Result<i64, String> {
    match &statement.kind(index) {
        Type::Integer => match statement.read::<i64>(index) {
            Ok(int_value) => Ok(int_value),
            Err(_) => Err(String::from(format!("Column {} not integer", &index)))
        },
        _ => {
            Err(String::from(format!("Column {} not integer", &index)))
        }
    }    
}

pub fn is_table_inited(connection: &sqlite::Connection, name: &str) -> bool {
    let cmd = format!("SELECT count(*) FROM {} WHERE table_name = '{}' and init > 0", CORE_INIT_TABLE, name);
    //println!("TABLE INIT CHECK: {}", &cmd);
    match connection.prepare( &cmd ) {
        Ok(mut s) => { // statement acquired
            match s.next() {
                Ok(sqlite::State::Row) => { // 
                    match s.read::<i64>(0) {
                        Ok(x) if x > 0 => {
                            println!("App_Init count: {}", &x);
                            true
                        },
                        Ok(_) => {
                            println!("App_Init doesn't contain {}", name);
                            false
                        },
                        _ => {
                            println!("Error reading app_init count column.");
                            false
                        }
                    }
                },
                _ => {
                    println!("Couldn't read a row");
                    false
                }
            }
        }, 
        Err(e) => {
            println!("Error prepping is_table_inited: {}", e);
            false
        }
    }
}

