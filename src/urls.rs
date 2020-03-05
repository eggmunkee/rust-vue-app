
use actix_web::{ web };
use crate::views;
use crate::views::{CrudModel};

//use crate::urls::app_urls;
use crate::core;
use crate::app;
use crate::survey;
//use crate::app::models::{User};

pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    // configure status url handling (defined in core::views)
    core::views::static_urls(cfg);    
    // Start configuring the service
    // include app urls
    app::urls::configure_urls(cfg);
    // include survey urls
    survey::urls::configure_urls(cfg);
}