// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use actix_cors::Cors;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// #[post("/clicked")]
// async fn clicked(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body("clicked")
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .wrap(Cors::permissive())
//             .service(hello)
//             .service(echo)
//             .service(clicked)
//             .route("/hey", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }


// use askama::Template; // bring trait in scope

// #[derive(Template)] // this will generate the code...
// #[template(path = "hello.html")] // using the template in this path, relative
//                                  // to the `templates` dir in the crate root
// struct HelloTemplate<'a> { // the name of the struct can be anything
//     title: &'a str, // the field name should match the variable name
//                    // in your template
// }

// fn main() {
//     let hello = HelloTemplate { title: "world" }; // instantiate your struct
//     println!("{}", hello.render().unwrap()); // then render it.
// }

use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpServer, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

async fn index(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .render()
        .expect("template should be valid")
    } else {
        Index.render().expect("template should be valid")
    };

    Ok(Html(html))
}

async fn clicked() -> Result<impl Responder> {
    Ok(Html(String::from("clicked")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/clicked").route(web::post().to(clicked)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}