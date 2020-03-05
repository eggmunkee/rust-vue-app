use actix_web::{ web };
use crate::views::CrudModel;
use crate::app::views;
use crate::app::models::{User};

// add routes to app views
pub fn configure_urls(cfg: &mut web::ServiceConfig) {

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

    cfg.service(web::resource("/app/").to(views::app_index));

    
    User::configure_model_crud(cfg);
}

