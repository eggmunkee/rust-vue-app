use actix_web::{middleware, web, App, HttpServer};
use std::sync::{Mutex};

// Register modules
mod macros;
mod core;
mod urls;
mod views;
mod db;
mod app_context;
mod app;

use crate::app_context::{AppContext,AppInstanceContext};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    // initialize database schema if needed
    db::init_db();

    // Construct shared state object
    let state = web::Data::new(AppContext {
        requests: Mutex::new(0)
    });

    // Create server app factory
    let future_server = HttpServer::new(move || {
        // create new app instance with a clone of the state data
        App::new().app_data(state.clone())
        // enable logger middleware
        .wrap(middleware::Logger::default())
        // hooks to configure app urls and fallback url view
        .configure(urls::configure_urls)
        .default_service( web::route().to(views::default_404) )
    })
    // Bind to ips/ports
    .bind("127.0.0.1:8080")?
    .bind("192.168.0.100:8080")?
    // Finalize
    .run();
    // Block on async server function to run it until further notice
    match future_server.await {
        Ok(_) => { Ok(()) },
        Err(e) => { println!("Server error: {}", e); Ok(()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        // let app = App::new().route("/", web::get().to(views::index));
        // let mut app = test::init_service(app).await;

        // let req = test::TestRequest::get().uri("/").to_request();
        // let resp = app.call(req).await.unwrap();

        // assert_eq!(resp.status(), http::StatusCode::OK);

        // let response_body = match resp.response().body().as_ref() {
        //     Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        //     _ => panic!("Response error"),
        // };

        // assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}