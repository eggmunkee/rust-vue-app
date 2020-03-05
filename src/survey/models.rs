
use sqlite::{State,Statement};

use serde::{Serialize,Deserialize};
use serde_json::json;

use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};

use crate::db::{get_conn};
use crate::core::models::{is_table_inited,add_init_table,prep_statement};
use crate::views::{CrudModel,CrudListQuery};
use crate::macros::*;
use crate::app_context::{AppContext,AppInstanceContext};

#[derive(Debug,Serialize,Deserialize)]
pub struct Question {
    pub id: i64,
    pub name: String,
}


impl CrudModel for Question {
    fn model_name() -> String {
        String::from("questions")
    }
    fn get(path: web::Path<(i64,)>, _req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
        data.log_request();
        let mut app_instance = AppInstanceContext::new();
        // let prep_statement borrow app instance to get boxed statement
        let boxed_statement = prep_statement(&mut app_instance, "SELECT rowid, * FROM questions WHERE rowid = ?");
        match boxed_statement  {
            Ok(statement) => {
                let mut statement = *statement; // deference boxed statement returned from prep_statement
                statement.bind(1, path.0).unwrap();
                println!("get Question id: {}", path.0);

                if let State::Row = statement.next().unwrap() {
                    HttpResponse::Ok().json( Question {
                        id: statement.read::<i64>(0).unwrap(),
                        name: statement.read::<String>(1).unwrap(),
                        //age: statement.read::<i64>(2).unwrap(),
                    })
                }
                else {
                    HttpResponse::NotFound().json(Question {
                        id: -1,
                        name: String::from("Not found."),
                        //age: 0
                    })
                }            
            },
            Err(_) => {
                HttpResponse::InternalServerError().json(Question {
                    id: -1,
                    name: String::from("Query error."),
                    //age: 0
                })
            }
        }
    }
    fn list(_query: web::Query<CrudListQuery>, _req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
        data.log_request();

        let mut app_instance = AppInstanceContext::new();
        // let prep_statement borrow app instance to get boxed statement
        let boxed_statement = prep_statement(&mut app_instance, "SELECT rowid, * FROM questions ORDER BY name");
        //match boxed_statement  {
        if let Ok(boxed_statement) = boxed_statement {
            println!("get Questions:");
            let mut users = Vec::<Question>::new();
            let mut statement = *boxed_statement;
            while let State::Row = statement.next().unwrap() {
                users.push(Question {
                    id: statement.read::<i64>(0).unwrap(),
                    name: statement.read::<String>(1).unwrap(),
                    //age: statement.read::<i64>(2).unwrap(),
                });
            }

            HttpResponse::Ok().json(users)
        }
        else {
            HttpResponse::InternalServerError().json(Question {
                id: -1,
                name: String::from("Query error."),
                //age: 0
            })
        }
    }
    // fn create(_query: web::Query<CrudListQuery>, _req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
    //     data.log_request();

    //     let mut app_instance = AppInstanceContext::new();
    //     // let prep_statement borrow app instance to get boxed statement
    //     let boxed_statement = prep_statement(&mut app_instance, 
    //         "insert into users values (?,?); select rowid, * from users order by rowid desc limit 1;");
    //     //match boxed_statement  {
    //     if let Ok(boxed_statement) = boxed_statement {
    //         println!("Add user:");
    //         let mut users = Vec::<User>::new();
    //         let mut statement = *boxed_statement;
    //         statement.bind(1, )
    //         while let State::Row = statement.next().unwrap() {
    //             users.push(User {
    //                 rowid: statement.read::<i64>(0).unwrap(),
    //                 name: statement.read::<String>(1).unwrap(),
    //                 age: statement.read::<i64>(2).unwrap(),
    //             });
    //         }

    //         HttpResponse::Ok().json(users)
    //     }
    //     else {
    //         HttpResponse::InternalServerError().json(User {
    //             rowid: -1,
    //             name: String::from("Query error."),
    //             age: 0
    //         })
    //     }
    // }
    fn configure_model_crud(cfg: &mut web::ServiceConfig) {
        // base route on model name
        let model_name : String = Question::model_name();
        cfg.service(web::scope(&model_name.as_str())
            .route("/{id}/", web::get().to(Question::get))
            .route("/", web::get().to(Question::list))
            .route("/", web::post().to(Question::list))
            .route("/{id}/", web::put().to(Question::list)));
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Answer {
    pub id: i64,
    pub name: String,
}

pub fn init_survey_models(connection: &mut sqlite::Connection) {

    if is_table_inited(&connection, "questions") == false {
        // Add users as inited
        if let Err(_) = add_init_table(&connection, "questions") {
            println!("Error adding init table");
        }
        // Create users table if doesn't exist    
        let mut cmd = create_table!("questions", "name TEXT");
        //columns!(col_text!("name"), col_int!("age"))
        match connection.execute( &cmd ) {
            Ok(_) =>  { },
            Err(e) => {
                println!("Error creating questions table: {}", e);
            }
        };
        // Create initial data for users table
        println!("Creating questions initial data...");
        cmd = format!("{} {}", 
            insert_row!("questions","'Age Question'"),
            insert_row!("questions","'Gender Question'").as_str() );
        match connection.execute( &cmd ) {
            Ok(_) =>  { println!("questions data inserted."); },
            Err(e) => {
                println!("questions init Error: {}\n Cmd: {}", e, &cmd);
            }
        }
    }
    else {
        println!("Skipping init questions...");
    }

    // if is_table_inited(&connection, "saved_urls") == false {
    //     // Add users as inited
    //     if let Err(_) = add_init_table(&connection, "saved_urls") {
    //         println!("Error adding init table");
    //     }
    //     // Create users table if doesn't exist    
    //     let mut cmd = create_table!("saved_urls","url_id INTEGER PRIMARY KEY ASC, name TEXT, url TEXT, favorite INTEGER" );
    //         // col_int!("favorite")));
    //     match connection.execute( &cmd ) {
    //         Ok(_) =>  { },
    //         Err(e) => {
    //             println!("Error creating saved_urls table: {}", e);
    //         }
    //     };
    //     // Create initial data for users table
    //     println!("Creating saved_urls initial data...");
    //     cmd = format!("{} {}", 
    //         insert_row_cols!("saved_urls","name, url, favorite", format!("'{}','{}',{}",&"url1", &"https://url.io", &1)),
    //         insert_row_cols!("saved_urls","name, url, favorite", format!("'{}','{}',{}",&"url2", &"https://url2.io", &0))
    //     );
    //     match connection.execute( &cmd ) {
    //         Ok(_) =>  { println!("Saved URLs [inserted]."); },
    //         Err(e) => {
    //             println!("Saved URLs init Error: {}\n Cmd: {}", e, &cmd);
    //         }
    //     }
    // }
    // else {
    //     println!("Skipping init saved urls...");
    // }
        
        
    // }
    // else {
    //     println!("Users already inited...");
    // }

    // let insert = insert_row!("users",columns!( val_text!("Alicia"), val_int!(36) ));
    // println!("Insert statement: {}", insert);
}

// INSERT INTO users VALUES ('Alice', 42);
// INSERT INTO users VALUES ('Bob', 69);


// // Get user count
// pub fn get_question_count<'a>(connection: &mut sqlite::Connection) -> i64 {
//     //let mut connection = get_conn();
//     println!("About to query users for count...");
//     let statement = connection.prepare("SELECT count(*) FROM users").unwrap();

//     statement.read::<i64>(0).unwrap()
// }

// pub fn get_users(connection: &mut sqlite::Connection) -> Vec::<User> {
//     //let mut connection = get_conn();
//     println!("About to query users...");
//     let prepped_statement = connection.prepare("SELECT rowid, * FROM users");
//     if let Ok(mut statement) = prepped_statement {
//         let mut users = Vec::new();
//         println!("get_users processing rows...");

//         while let State::Row = statement.next().unwrap() {
//             println!("name = {}", statement.read::<String>(0).unwrap());
//             println!("name = {}", statement.read::<String>(1).unwrap());
//             println!("age = {}", statement.read::<i64>(2).unwrap());
//             // read field vars
//             let rowid : i64 = statement.read::<i64>(0).unwrap();
//             let name : String = statement.read::<String>(1).unwrap();
//             //let name = ;
//             //let name_str = name.as_str();
//             let age = statement.read::<i64>(2).unwrap();        
//             let user = User {
//                 rowid, name, age
//             };
//             users.push(user);
//         }
        
//         users
//     }
//     else {
//         println!("Failed to query users.");
//         Vec::new()
//     }

    
// }