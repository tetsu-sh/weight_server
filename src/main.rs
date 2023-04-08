use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::TimeZone;
use chrono::{Local, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use utils::errors::MyError;
use utils::state::AppState;
mod constants;
mod schema;
mod store;
mod utils;

#[macro_use]
extern crate diesel;

#[get("/")]
async fn hello() -> impl Responder {
    println!("hello-[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[");
    HttpResponse::Ok().body("Hello world!")
}
pub type ApiResponse = Result<HttpResponse, MyError>;

#[post("/weight")]
async fn weight(state: web::Data<AppState>, form: web::Json<Request>) -> ApiResponse {
    let conn = state.get_db_conn()?;
    store::save(&conn, form.weight, form.timestamp, form.device_id)?;
    Ok(HttpResponse::Ok().json({}))
}

#[derive(Deserialize, Serialize, Debug)]
struct Request {
    timestamp: i64,
    weight: f32,
    device_id: i32,
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
