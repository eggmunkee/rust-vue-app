
use actix_web::{ web };
use crate::views;

//use crate::urls::app_urls;
use crate::core;
use crate::app;


pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    core::views::static_urls(cfg);
    // hardcoded return content:    
    //.service(web::resource("/static/{url}").to(core::views::static_content))
    // Index
    cfg.service(web::resource("/").to(views::index))
    .service(web::resource("/index.html").to(views::index))
    .service(web::resource("/with_both/{item}/").to(views::with_both))
    .service(web::resource("/with_path/").to(views::with_query))
    .service(web::resource("/with_path/{item}/{item2}").to(views::with_path))
    // about
    .service(web::resource("/about.html").to(|| async { "Rust Vue App, built with actix_web!" }));

    // include app urls
    app::urls::configure_urls(cfg);
    // other module urls add below
}