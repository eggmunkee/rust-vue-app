use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::dev::{ Body };

use serde::Deserialize;

use tera::{Context};

use crate::app::models::{get_users};
use crate::core::utils::{get_template_context};
use crate::db::{get_conn};
use crate::app_context::{AppContext};

// define query string params for Crud List operations (for paging, etc.)
#[derive(Deserialize)]
pub struct CrudListQuery {
    offset: Option<usize>
}

// Definte a trait for generic handling of Crud apis for models, the urls and views which respond
pub trait CrudModel {
    fn model_name() -> String;
    fn get(path: web::Path<(i64,)>, req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse;
    fn list(query: web::Query<CrudListQuery>, req: HttpRequest, data: web::Data<AppContext>) -> HttpResponse;
    fn configure_model_crud(cfg: &mut web::ServiceConfig);
}

// TODO: generalize routes portion using template with trait CrudModel
// borrow checker not happy with current code
// pub fn configure_model_crud(cfg: &mut web::ServiceConfig) 
//     where T: CrudModel {
//     // base route on model name
//     let model_name : String = T::model_name();
//     cfg.service(web::scope(&model_name.as_str())
//         .route("/{id}/", web::get().to(User::get)));
// }


// TESTING PATH AND QUERY 
// Define query param data structure
#[derive(Deserialize)] // Allow serde to deserialize struct from query
pub struct WithQueryInfo {
    pub item: String, // required query param
    pub optional: Option<String>, // optional query param
}



pub async fn default_404(req: HttpRequest) -> HttpResponse {
    // For a simple response body
    // Build response body with a String object
    let mut msg = String::from("<!doctype html>
    <html>
        <head>
            <link rel=\"stylesheet\" type=\"text/css\" href=\"/static/css/base.css\">
        </head>
        <body>
        <h1>That isn't here.</h1>
        <p>Try <a href=\"/\">here</a>.</p>
        <h1>Attempted url:
    ");
    // include path manually
    msg.push_str(req.path());
    // footer
    msg.push_str("</h1></body>
    </html>");
    // return string response body
    HttpResponse::Ok().body(msg)
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