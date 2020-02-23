use actix_web::{HttpRequest,HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::dev::{ Body };


use crate::app::models::{Question, Answer};

use crate::db::{ get_conn };
use crate::app::models::{ get_users };
use crate::core::utils::{get_template_context};
use crate::views::{get_user_context};

pub async fn index<'a>(_req: HttpRequest) -> HttpResponse {
    let (tera, mut context) = get_template_context().unwrap();

    get_user_context(&mut context);

    let _q = Question {
        id: 1, name: "test"
    };
    let ans_name = format!("{}", "abc");
    let _a = Answer { id: 1000, name: &ans_name.as_str() };
    //println!("REQ: {:?}", req);
    
    // Create a path to the desired file
    // let path = Path::new("./templates/base.html");
    // let display = path.display();
    // let file_name = path.file_name().unwrap().to_str().unwrap();

    let mut conn = get_conn();
    get_users(&mut conn);

    let page_source = match tera.render("app/index.html", &context) {
        Ok(r) => r,
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    //if let Ok(body_string) = res {
    HttpResponse::with_body(StatusCode::OK, Body::from_message(page_source))
    

    // open file read-only
    // let mut file = match File::open(&path) {
    //     // The `description` method of `io::Error` returns a string that
    //     // describes the error
    //     Err(why) => panic!("couldn't open {}: {}", display,
    //                                                why.description()),
    //     Ok(file) => file,
    // };

    // let mut body_string = String::new();

    //body_string.push_str(format!("display:{}, filename:{}", display, file_name).as_str());
    // match file.read_to_string(&mut body_string) {
    //     Err(why) => panic!("couldn't read {}: {}", display,
    //                                                why.description()),
    //     Ok(_) => print!("{} contains:\n{}", display, body_string),
    // }

    //HttpResponse::with_body(StatusCode::OK, Body::from_message(body_string))
}