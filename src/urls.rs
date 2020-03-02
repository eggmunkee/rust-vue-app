
use actix_web::{ web };
use crate::views;
use crate::views::{CrudModel};

//use crate::urls::app_urls;
use crate::core;
use crate::app;
use crate::app::models::{User};

pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    // configure status url handling (defined in core::views)
    core::views::static_urls(cfg);    
    // Start configuring the service
    cfg
    // two aliases to index view
    .service(web::resource("/").to(views::index))
    .service(web::resource("/index.html").to(views::index))
    // Routing test urls
    .service(web::resource("/with_both/{item}/").to(views::with_both))
    .service(web::resource("/with_path/").to(views::with_query))
    .service(web::resource("/with_path/{item}/{item2}").to(views::with_path))
    // about
    .service(web::resource("/about.html").to(views::about));
    // include app urls
    app::urls::configure_urls(cfg);
    // configure urls for User model CRUD api
    User::configure_model_crud(cfg);
}