use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use rusqlite::Connection;

use crate::sql;

#[get("/")]
async fn hello() -> impl Responder {
    let mut conn = Connection::open("banki.db").unwrap();
    let mut payments = sql::get_payments(&mut conn).unwrap();
    println!("{:?}", payments);
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
