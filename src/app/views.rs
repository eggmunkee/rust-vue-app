use actix_web::{HttpRequest};

pub async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "/app/ Hello world!"
}