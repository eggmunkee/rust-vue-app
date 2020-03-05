use actix_web::{
    web,HttpRequest,HttpResponse,
    http::{StatusCode},
    dev::{ Body }
};
use serde_json::{json};
use tera::{Context};

//use crate::app::models::{Question, Answer};
use crate::app_context::{AppContext,AppInstanceContext};
use crate::db::{ get_conn };
use crate::app::models::{ get_users };
use crate::core::utils::{get_template_context};
use crate::views::{WithQueryInfo};


pub fn get_user_context(context: &mut Context) {
    // Get db connection and users list
    let mut conn = get_conn();
    let users = get_users(&mut conn);
    drop(conn); // drop connection memory

    // fill in context with users information
    context.insert("users", &users);
    context.insert("user_count", &users.len());
    // serialize users list into context
    if let Ok(users_json) = serde_json::to_string(&users) {
        context.insert("users_json", &users_json);
    }
    else {
        context.insert("users_json", &"[]");
    }
}


// Require WithQueryInfo and a single url path placeholder String
// invalid: base/url/any_val/
// invalid: base/url/any_value/?optional=123
// valid: base/url/_any__value_/?item=item_val
// valid: base/url/_any__value_/?item=item_val&optional=opt_val
//  In above examples, the path String = "_any__value_", query item would be "item_val",
//    and in optional example, query.optional is Some("opt_val")
pub async fn with_both(path: web::Path<String>, query: web::Query<WithQueryInfo>) -> HttpResponse {
    match get_template_context() {
        Ok((tera, mut context)) => {
            let path = &path.as_str();
            let item = &query.item.as_str();
            let resp_form = match &query.optional {
                Some(opt) => format!("path: {}, item value: {}, optional: {}", &path, &item, &opt),
                _ => format!("path: {}, item value: {}", &path, &item)
            };
            println!("resp_form: {}", &resp_form);
            return HttpResponse::with_body(StatusCode::OK, Body::from_message(resp_form))
        },
        _ => {
            return HttpResponse::with_body(StatusCode::OK, Body::from_message("error"))
        }
    }
}

pub async fn with_query(query: web::Query<WithQueryInfo>) -> HttpResponse {
    match get_template_context() {
        Ok((tera, mut context)) => {
            let item = &query.item.as_str();
            let resp_form = format!("item value: {}", &item);
            println!("resp_form: {}", &resp_form);
            return HttpResponse::with_body(StatusCode::OK, Body::from_message(resp_form))
        },
        _ => {
            return HttpResponse::with_body(StatusCode::OK, Body::from_message("error"))
        }
    }
}

pub async fn with_path(info: web::Path<(String,String)>) -> HttpResponse {
    // 
    match get_template_context() {
        Ok((tera, mut context)) => {
            let item = &info.0;
            let item2 = &info.1;
            let resp_form = format!("item value: {} /// {}", &item, &item2);
            println!("resp_form: {}", &resp_form);
            return HttpResponse::with_body(StatusCode::OK, Body::from_message(resp_form))
        },
        _ => {
            return HttpResponse::with_body(StatusCode::OK, Body::from_message("error"))
        }
    }
}

pub async fn index(_req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
    let (tera, mut context) = get_template_context().unwrap();
    // log request and save result in context    
    context.insert("requests", &data.log_request());
    // add user context information into the context object
    self::get_user_context(&mut context);
    // try to render the index template with context
    let rendered = match tera.render("index.html", &context) {
        Ok(r) => r,
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // return rendered template as 200 - Ok response
    HttpResponse::Ok().body(rendered)
}

pub async fn about(_req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
    // Get the template engine and template context
    let (tera, mut context) = get_template_context().unwrap();
    // Insert # of requests into template context
    context.insert("requests", &data.log_request());
    // render template or process template error
    let rendered = match tera.render("about.html", &context) {
        Ok(r) => r, // set rendered = to the result r
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    // Return Ok response with rendered template
    HttpResponse::Ok().body(rendered)
}


pub async fn app_index<'a>(_req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse {
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

