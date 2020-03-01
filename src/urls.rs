
use actix_web::{ web };
use crate::views;
use crate::views::{CrudModel};

//use crate::urls::app_urls;
use crate::core;
use crate::app;
use crate::app::models::{User};

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
    //let user = User::new();
    User::configure_model_crud(cfg);
}

// pub fn configure_model_crud<T>(model_name: &str, cfg: &mut web::ServiceConfig) 
//     where T: CrudModel {
//     // base route on model name
//     let mod_name : String = String::from(model_name);
//     cfg.service(web::scope(model_name)
//         .route("/{id}/", web::get().to(&T::get)));
// }