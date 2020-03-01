use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::dev::{ Body };


use serde::Deserialize;

// file reading requirements

use tera::{Context};

use crate::app::models::{get_users};
use crate::core::utils::{get_template_context};
use crate::db::{get_conn};
use crate::app_context::{AppContext};

#[derive(Deserialize)]
pub struct CrudListQuery {
    offset: Option<usize>
}

pub trait CrudModel {
    fn model_name() -> String;
    fn get(path: web::Path<(i64,)>, req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse;
    fn list(query: web::Query<CrudListQuery>, req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse;
    fn configure_model_crud(cfg: &mut web::ServiceConfig);
}


// pub fn configure_model_crud(cfg: &mut web::ServiceConfig) 
//     where T: CrudModel {
//     // base route on model name
//     let model_name : String = T::model_name();
//     cfg.service(web::scope(&model_name.as_str())
//         .route("/{id}/", web::get().to(User::get)));
// }


pub fn get_user_context(context: &mut Context) {
    // Get users list and create json for list
    let mut conn = get_conn();
    
    let users = get_users(&mut conn);
    context.insert("users", &users);
    //let users = get_user_list();   
    context.insert("user_count", &users.len());
    //context.insert("users", &users);
    if let Ok(users_json) = serde_json::to_string(&users) {
        context.insert("users_json", &users_json);
    }
    else {
        context.insert("users_json", &"[]");
    }
    
}

// TESTING PATH AND QUERY 
// Define query param data structure
#[derive(Deserialize)] // Allow serde to deserialize struct from query
pub struct WithQueryInfo {
    pub item: String, // required query param
    pub optional: Option<String>, // optional query param
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
    // get database connection
    //let mut conn = get_conn();
    println!("Before requests lock");
    // 
    // if let Ok(mut requests) = data.requests.lock() {
    //     *requests += 1;
    //     context.insert("requests", &*requests);           
    // }
    // else {
    //     context.insert("requests", "-1");           
    // }
    context.insert("requests", &data.log_request());

    println!("After requests lock");
    self::get_user_context(&mut context);

    //tera.add_template_file(Path::new("./templates/base.html"), Some("base"));
    let res = match tera.render("index.html", &context) {
        Ok(r) => r,
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    //if let Ok(body_string) = res {
    HttpResponse::with_body(StatusCode::OK, Body::from_message(res))
    //} else if let Error(err) = res {
        //HttpResponse::with_body(StatusCode::OK, Body::from_message(format!("Error rendering base.html: {}", err)))
    //}

    

    //let user_size = get_users();

    //let resp = format!("/ Hello world! User count: {}", &0);

    //resp
}

pub async fn default_404(req: HttpRequest) -> HttpResponse {
    // Build response body
    let mut msg = String::from("<!doctype html>
    <html>
        <body>
        <h1>That ain't here, or here isn't that.</h1>
        <p>Try <a href=\"/\">here</a>.</p>
        <h1>Attempted url:
    ");
    // include path
    msg.push_str(req.path());
    // footer
    msg.push_str("</h1></body>
    </html>");

    HttpResponse::with_body(StatusCode::OK, Body::from_message(msg))
}


/*
// Create a path to the desired file
    let path = Path::new(format!("./static/{}", url).as_str());
    let display = path.display();
    let file_name = path.file_name().unwrap().to_str().unwrap();


    // open file read-only
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut body_string = String::new();

    //body_string.push_str(format!("display:{}, filename:{}", display, file_name).as_str());
    match file.read_to_string(&mut body_string) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, body_string),
    }
*/