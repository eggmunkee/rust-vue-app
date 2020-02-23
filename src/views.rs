use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};
use actix_web::dev::{ Body };

// file reading requirements

use tera::{Context};

use crate::app::models::{get_users};
use crate::core::utils::{get_template_context};
use crate::db::{get_conn};

pub fn get_user_context(context: &mut Context) {
    // Get users list and create json for list
    let mut conn = get_conn();
    let users = get_users(&mut conn);
    context.insert("users", &users);
    //let users = get_user_list();   
    context.insert("user_count", &users.len());
    //context.insert("users", &users);
    if let Ok(users_json) = serde_json::to_string(&users) {
        context.insert("users_json", &users_json);
    }
    else {
        context.insert("users_json", &"[]");
    }
    
}

pub async fn index(_req: HttpRequest) -> HttpResponse {
    let (tera, mut context) = get_template_context().unwrap();
    // get database connection
    //let mut conn = get_conn();
    
    // let user_count = get_user_count(&mut conn);
    // context.insert("user_count", &user_count);

    self::get_user_context(&mut context);

    //tera.add_template_file(Path::new("./templates/base.html"), Some("base"));
    let res = match tera.render("index.html", &context) {
        Ok(r) => r,
        Err(e) => { 
            println!("Render error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    //if let Ok(body_string) = res {
    HttpResponse::with_body(StatusCode::OK, Body::from_message(res))
    //} else if let Error(err) = res {
        //HttpResponse::with_body(StatusCode::OK, Body::from_message(format!("Error rendering base.html: {}", err)))
    //}

    

    //let user_size = get_users();

    //let resp = format!("/ Hello world! User count: {}", &0);

    //resp
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


/*
// Create a path to the desired file
    let path = Path::new(format!("./static/{}", url).as_str());
    let display = path.display();
    let file_name = path.file_name().unwrap().to_str().unwrap();


    // open file read-only
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut body_string = String::new();

    //body_string.push_str(format!("display:{}, filename:{}", display, file_name).as_str());
    match file.read_to_string(&mut body_string) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, body_string),
    }
*/