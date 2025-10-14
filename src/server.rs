use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
