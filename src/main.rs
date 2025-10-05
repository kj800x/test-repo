use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Lets gooo!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://0.0.0.0:8080");
    println!("Hello World!");

    for (env, value) in std::env::vars() {
        println!("{}: {}", env, value);
    }

    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
