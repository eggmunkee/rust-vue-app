use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::dev::{ Body };

// file reading requirements
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn load_static_url<'a>(url: &str) -> String {
    let url_formatted = format!("./static/{}", url);
    let path = Path::new(url_formatted.as_str());
    let display = path.display();

    // open file read-only
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut body_string = String::new();

    //body_string.push_str(format!("display:{}, filename:{}", display, file_name).as_str());
    match file.read_to_string(&mut body_string) {
        Ok(_) => (),
        Err(why) => panic!("couldn't read {}: {}", display, why),        
    }

    body_string
}

pub async fn static_content<'a>(req: HttpRequest) -> HttpResponse {
    // get parameters from request
    let match_info = req.match_info();
    let url = match_info.query("url");
    
    let body_string = self::load_static_url(url);

    let mut response = HttpResponse::with_body(StatusCode::OK, Body::from_message(body_string));

    let hdrs = response.headers_mut();

    match url {
        x if x.ends_with(".json") => {
            hdrs.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        x if x.ends_with(".js") => {
            hdrs.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/javascript;charset=UTF-8"));
        },
        x if x.ends_with(".css") => {
            hdrs.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/css;charset=UTF-8"));
        },
        x if x.ends_with(".json") => {
            hdrs.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        _ => ()
    }
    
    response
}

pub fn static_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/static/{url:[^{}]+}").to(self::static_content));
}