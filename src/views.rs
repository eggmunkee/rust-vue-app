use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::dev::{ Body };

pub async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "/ Hello world!"
}

pub async fn default_404(req: HttpRequest) -> HttpResponse {
    // Build response body
    let mut msg = String::from("<!doctype html>
    <html>
        <body>
        <h1>That ain't here, or here isn't that.</h1>
        <p>Try <a href=\"/\">here</a>.</p>
        <h1>Attempted url:
    ");
    // include path
    msg.push_str(req.path());
    // footer
    msg.push_str("</h1></body>
    </html>");

    HttpResponse::with_body(StatusCode::OK, Body::from_message(msg))
}