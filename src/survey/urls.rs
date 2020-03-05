use actix_web::{ web };

use crate::views::CrudModel;

// add routes to app views
pub fn configure_urls(cfg: &mut web::ServiceConfig) {
    use crate::survey::models::{Question};

    Question::configure_model_crud(cfg);
}

