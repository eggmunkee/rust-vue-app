
use sqlite::{State,Statement};

use serde::{Serialize,Deserialize};

use crate::core::models::{is_table_inited,add_init_table};

#[derive(Debug,Serialize,Deserialize)]
pub struct User {
    pub name: String,
    pub age: i64
}

#[derive(Debug)]
pub struct Question<'a> {
    pub id: i32,
    pub name: &'a str,
}
#[derive(Debug)]
pub struct Answer<'a> {
    pub id: i32,
    pub name: &'a str,
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

pub fn init_users(connection: &mut sqlite::Connection) {

    if is_table_inited(&connection, "users") == false {
        // Add users as inited
        if let Err(_) = add_init_table(&connection, "users") {
            println!("Error adding init table");
        }
        // Create users table if doesn't exist    
        let mut cmd = create_table!("users",columns!(col_text!("name"),col_int!("age")));
        match connection.execute( &cmd ) {
            Ok(_) =>  { },
            Err(e) => {
                println!("Error creating users table: {}", e);
            }
        };
        // Create initial data for users table
        println!("Creating users initial data...");
        cmd = format!("{} {}", insert_row!("users",columns!( val_text!("Alicia"), val_int!(36) )),
         insert_row!("users",columns!( val_text!("Noah"), val_int!(37) )).as_str() );
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
    let prepped_statement = connection.prepare("SELECT * FROM users");
    if let Ok(mut statement) = prepped_statement {
        println!("Some() prepped statement.");
        let mut users = Vec::new();
        println!("get_users");

        while let State::Row = statement.next().unwrap() {
            println!("name = {}", statement.read::<String>(0).unwrap());
            println!("age = {}", statement.read::<i64>(1).unwrap());
            let name : String = statement.read::<String>(0).unwrap();
            //let name = ;
            //let name_str = name.as_str();
            let age = statement.read::<i64>(1).unwrap();        
            let user = User {
                name: name,
                age: age
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