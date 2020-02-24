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
    
    let mut conn = get_conn();
    get_users(&mut conn);

    let page_source = match tera.render("app/index.html", &context) {
        Ok(r) => r,
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    //return 
    HttpResponse::with_body(StatusCode::OK, Body::from_message(page_source))    
}

