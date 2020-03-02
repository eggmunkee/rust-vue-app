use actix_web::{
    web,HttpRequest,HttpResponse,
    http::{StatusCode},
    dev::{ Body }
};
use serde_json::{json};

use crate::app::models::{Question, Answer};
use crate::app_context::{AppContext,AppInstanceContext};
use crate::db::{ get_conn };
use crate::app::models::{ get_users };
use crate::core::utils::{get_template_context};
use crate::views::{get_user_context};

pub async fn index<'a>(_req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
    let (tera, mut context) = get_template_context().unwrap();

    get_user_context(&mut context);

    context.insert("requests", &data.log_request());

    // Get app instance with database connection    
    let mut app_instance = AppInstanceContext::new();
    let mut conn = app_instance.connection;
    // get users
    let users = get_users(&mut conn);
    context.insert("users", &serde_json::to_string(&users).unwrap());

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

