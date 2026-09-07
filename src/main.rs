use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    let mut vars: Vec<String> = std::env::vars()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    vars.sort();

    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!("Hello big world\n\n{}", vars.join("\n")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Heya World!");

    for (env, value) in std::env::vars() {
        println!("{}: {}", env, value);
    }

    if std::env::var("IS_A_CRON_JOB").is_ok() {
        for i in 0..10 {
            println!("Running job i={}", i);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        Ok(())
    } else {
        println!("Server starting at http://0.0.0.0:8080");
        HttpServer::new(|| App::new().service(hello))
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
    }
}
