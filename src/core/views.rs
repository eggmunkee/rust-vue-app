use actix_web::{
    web,
     
    HttpRequest, HttpResponse,
    http::{
        StatusCode,
        header::{HeaderName, HeaderValue}
    },
    dev::{ Body }
};
use actix_http::{ResponseBuilder};
// use actix_web::;
// use actix_web::http::header::{HeaderName, HeaderValue};
// use actix_web::dev::{ Body };
use bytes::{BytesMut,BufMut};
use futures::stream::*;
use futures::future::ok;

// file reading requirements
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//fn load_static

fn load_static_url_string<'a>(url: &str) -> String {
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

struct LoadFlags {
    allow: bool,
    load_string: bool
}

fn process_url_load_flags(flags: &mut LoadFlags, rb: &mut ResponseBuilder) {

}

pub async fn static_content<'a>(req: HttpRequest) -> HttpResponse {
    // get parameters from request
    let match_info = req.match_info(); // get match info about request url and route
    let url = match_info.query("url"); // get url param by name
    
    // Create ok response, and see if the url is a statis loadable asset
    let mut rb = HttpResponse::Ok();

    // Set content type headers and whether the url can be loaded
    let mut allow = true;
    let mut load_string = true;
    match url {
        x if x.ends_with(".json") => {
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        x if x.ends_with(".js") => {
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("text/javascript;charset=UTF-8"));
        },
        x if x.ends_with(".css") => {
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("text/css;charset=UTF-8"));
        },
        x if x.ends_with(".json") => {
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("application/json"));
        },
        x if x.ends_with(".jpg") || x.ends_with(".jpeg") => {
            load_string = false; // load as bytes
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("image/jpeg"));
        },
        x if x.ends_with(".png") => {
            load_string = false; // load as bytes
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("image/png"));
        },
        x if x.ends_with(".mp3") => {
            load_string = false; // load as bytes
            rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("audio/mpeg"));
        },
        // x if x.ends_with(".mp3") => {
        //     rb.header(HeaderName::from_static("content-type"), HeaderValue::from_static("audio/mpeg"));
        // },
        _ => {
            allow = false;
        }
    }

    match allow {
        true => match load_string {
            true => {
                let body_string = self::load_static_url_string(url);
                rb.body(body_string)
            },
            _ => {
                let mut file : std::fs::File;
                let url_formatted = format!("./static/{}", url);
                let path = Path::new(url_formatted.as_str());
                match File::open(&path) {
                    Ok(f) => { file = f; },
                    _ => {
                        return rb.body("Nein");
                    }
                }
                
                let mut bytes = BytesMut::with_capacity(8192);
                let mut buffer = [0; 8192];

                // chunks of the file and build bytes in memory
                while let Ok(n) = file.read(&mut buffer) {
                    if n > 0 {
                        bytes.put(&buffer[..n]);
                        //println!("Read bytes: {}", &n);
                    }
                    else {
                        //println!("Read ZERO, stopping");
                        break;
                    }
                }
                // dump bytes to response
                // TODO: improve by making body a repeated streaming type
                let body = once(ok::<_, std::io::Error>(bytes.freeze()));
                rb.streaming(body)
            }
        },
        _ => {
            rb.body("Nein")
        }
    }

    
}

pub fn static_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/static/{url:[^{}]+}").to(self::static_content));
}