use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Local;
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
mod constants;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    println!("hello-[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/weight")]
async fn weight(form: web::Json<Request>) -> impl Responder {
    println!("{:?}", form);
    println!("{:?}", Local.timestamp_opt(form.timestamp as i64, 0));
    HttpResponse::Ok().body("Hey there!")
}
#[derive(Deserialize, Serialize, Debug)]
struct Request {
    timestamp: i32,
    weight: f32,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = utils::db::establish_connection();
    let app_state = utils::state::AppState { pool: pool };
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .service(hello)
            .service(weight)
    })
    .bind(("0.0.0.0", 8000))?
    .workers(1)
    .run()
    .await
}
