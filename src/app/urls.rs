use actix_web::{ web };

use crate::app::views::{ index };

// add routes to app views
pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/app/").to(index));
}

