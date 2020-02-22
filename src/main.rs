use actix_web::{middleware, web, App, HttpServer};

// Register modules
mod urls;
mod views;
mod app;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
        // enable logger
        .wrap(middleware::Logger::default())
        // urls hook
        .configure(urls::configure_urls)
        .default_service( web::route().to(views::default_404) )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
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