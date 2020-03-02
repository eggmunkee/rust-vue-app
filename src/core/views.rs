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

fn is_static_load_url(url : &str) -> bool {
    match url {
        x if x.ends_with(".mp3") => {
            false
        },
        _ => true
    }
}

pub async fn static_content<'a>(req: HttpRequest) -> HttpResponse {
    // get parameters from request
    let match_info = req.match_info(); // get match info about request url and route
    let url = match_info.query("url"); // get url param by name
    
    // Create ok response, and see if the url is a statis loadable asset
    let mut rb = HttpResponse::Ok();
    let can_static_load = is_static_load_url(&url);

    // Set content type headers and whether the url can be loaded
    let mut allow = false;
    match url {
        x if x.ends_with(".json") => {
            allow = true;
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        x if x.ends_with(".js") => {
            allow = true;
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("text/javascript;charset=UTF-8"));
        },
        x if x.ends_with(".css") => {
            allow = true;
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("text/css;charset=UTF-8"));
        },
        x if x.ends_with(".json") => {
            allow = true;
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        // x if x.ends_with(".mp3") => {
        //     rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("audio/mpeg"));
        // },
        _ => {
            allow = false;
        }
    }

    match allow {
        true => {
            let body_string = self::load_static_url(url);
            rb.body(body_string)
        },
        _ => {
            //let body_string = self::load_static_url(url);
            rb.body("")
        }
    }

    
}

pub fn static_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/static/{url:[^{}]+}").to(self::static_content));
}