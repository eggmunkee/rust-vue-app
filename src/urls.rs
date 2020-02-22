
use actix_web::{ web };
use crate::views;

//use crate::urls::app_urls;
use crate::app;


pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    // hardcoded return content:
    cfg.service(web::resource("/about.html").to(|| async { "Rust Vue App, built with actix_web!" }))
    .service(web::resource("/index.html").to(|| async { "Hello world!" }))
    // pass to request handler (view)
    .service(web::resource("/").to(views::index));
    // include app urls
    app::urls::configure_urls(cfg);
    // other module urls add below
}