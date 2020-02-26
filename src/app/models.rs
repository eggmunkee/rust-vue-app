
use sqlite::{State,Statement};

use serde::{Serialize,Deserialize};

use crate::core::models::{is_table_inited,add_init_table};
use crate::macros::*;

#[derive(Debug,Serialize,Deserialize)]
pub struct User {
    pub rowid: i64,
    pub name: String,
    pub age: i64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct SavedUrl {
    pub url_id: i64,
    pub name: String,
    pub url: String,
    pub favorite: i64,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Question {
    pub id: i32,
    pub name: String,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Answer {
    pub id: i32,
    pub name: String,
}

pub fn get_user_list() -> Vec<User> {
    let users : Vec<User> = Vec::new();

    // for x in &[1,2,3] {
    //     let user = User {
    //         name: "test",
    //         age: x*3
    //     };
    //     users.push(user);
    // }

    users
}

pub fn init_app_models(connection: &mut sqlite::Connection) {

    if is_table_inited(&connection, "users") == false {
        // Add users as inited
        if let Err(_) = add_init_table(&connection, "users") {
            println!("Error adding init table");
        }
        // Create users table if doesn't exist    
        let mut cmd = create_table!("users", "name TEXT, age INTEGER");
        //columns!(col_text!("name"), col_int!("age"))
        match connection.execute( &cmd ) {
            Ok(_) =>  { },
            Err(e) => {
                println!("Error creating users table: {}", e);
            }
        };
        // Create initial data for users table
        println!("Creating users initial data...");
        cmd = format!("{} {}", 
            insert_row!("users","'Alicia', 36"),
            insert_row!("users","'Noah', 37").as_str() );
        match connection.execute( &cmd ) {
            Ok(_) =>  { println!("2 Users [inserted]."); },
            Err(e) => {
                println!("Users init Error: {}\n Cmd: {}", e, &cmd);
            }
        }
    }
    else {
        println!("Skipping init users...");
    }

    if is_table_inited(&connection, "saved_urls") == false {
        // Add users as inited
        if let Err(_) = add_init_table(&connection, "saved_urls") {
            println!("Error adding init table");
        }
        // Create users table if doesn't exist    
        let mut cmd = create_table!("saved_urls","url_id INTEGER PRIMARY KEY ASC, name TEXT, url TEXT, favorite INTEGER" );
            // col_int!("favorite")));
        match connection.execute( &cmd ) {
            Ok(_) =>  { },
            Err(e) => {
                println!("Error creating saved_urls table: {}", e);
            }
        };
        // Create initial data for users table
        println!("Creating saved_urls initial data...");
        cmd = format!("{} {}", 
            insert_row_cols!("saved_urls","name, url, favorite", format!("'{}','{}',{}",&"Alicia''s url", &"https://www.twitter.com/AliciaF424", &1)),
            insert_row_cols!("saved_urls","name, url, favorite", format!("'{}','{}',{}",&"Noah - Banned.video", &"https://banned.video/", &1))
            //insert_row!("saved_urls",columns!(val_int!(1), val_text!("Alicia's url"), val_int!(36) )),
            //insert_row!("saved_urls",columns!(val_int!(2),val_text!("Noah - Banned.video"), val_text!("https://banned.video/"))).as_str() );
        );
        match connection.execute( &cmd ) {
            Ok(_) =>  { println!("Saved URLs [inserted]."); },
            Err(e) => {
                println!("Saved URLs init Error: {}\n Cmd: {}", e, &cmd);
            }
        }
    }
    else {
        println!("Skipping init users...");
    }
        
        
    // }
    // else {
    //     println!("Users already inited...");
    // }

    // let insert = insert_row!("users",columns!( val_text!("Alicia"), val_int!(36) ));
    // println!("Insert statement: {}", insert);
}

// INSERT INTO users VALUES ('Alice', 42);
// INSERT INTO users VALUES ('Bob', 69);


// Get user count
pub fn get_user_count<'a>(connection: &mut sqlite::Connection) -> usize {
    //let mut connection = get_conn();
    println!("About to query users for count...");
    let statement = connection.prepare("SELECT * FROM users").unwrap();

    statement.count()
}

pub fn get_users(connection: &mut sqlite::Connection) -> Vec::<User> {
    //let mut connection = get_conn();
    println!("About to query users...");
    let prepped_statement = connection.prepare("SELECT rowid, * FROM users");
    if let Ok(mut statement) = prepped_statement {
        println!("Some() prepped statement.");
        let mut users = Vec::new();
        println!("get_users");

        while let State::Row = statement.next().unwrap() {
            println!("name = {}", statement.read::<String>(0).unwrap());
            println!("name = {}", statement.read::<String>(1).unwrap());
            println!("age = {}", statement.read::<i64>(2).unwrap());
            // read field vars
            let rowid : i64 = statement.read::<i64>(0).unwrap();
            let name : String = statement.read::<String>(1).unwrap();
            //let name = ;
            //let name_str = name.as_str();
            let age = statement.read::<i64>(2).unwrap();        
            let user = User {
                rowid, name, age
            };
            users.push(user);
        }
        
        users
    }
    else {
        println!("Failed to query users.");
        Vec::new()
    }

    
}