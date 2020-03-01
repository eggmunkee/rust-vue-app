use actix_web::{
    web,HttpRequest,HttpResponse,
    http::{StatusCode},
    dev::{ Body }
};


use crate::app::models::{Question, Answer};
use crate::app_context::{AppContext};
use crate::db::{ get_conn };
use crate::app::models::{ get_users };
use crate::core::utils::{get_template_context};
use crate::views::{get_user_context};

pub async fn index<'a>(_req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
    let (tera, mut context) = get_template_context().unwrap();

    get_user_context(&mut context);

    context.insert("requests", &data.log_request());

    let _q = Question {
        id: 1, name: String::from("test")
    };
    let ans_name = format!("{}", "abc");
    let _a = Answer { id: 1000, name: ans_name.to_string() };
    
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

