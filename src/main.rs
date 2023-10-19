use actix_web::middleware::Logger;
use actix_web::web::get;
use actix_web::web::post;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
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

async fn hello() -> impl Responder {
    println!("hello-[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[[");
    HttpResponse::Ok().body("Hello world!")
}
pub type ApiResponse = Result<HttpResponse, MyError>;

async fn post_weight(
    state: web::Data<AppState>,
    form: web::Json<PostWeightRequest>,
) -> ApiResponse {
    let conn = state.get_db_conn()?;
    store::save(&conn, form.weight, form.timestamp, form.device_id)?;
    Ok(HttpResponse::Ok().json({}))
}

async fn get_weight(state: web::Data<AppState>, form: web::Query<GetWeightRequest>) -> ApiResponse {
    let conn = state.get_db_conn()?;
    println!("{}{}", form.datetime_from, form.datetime_to);
    let res = store::fetch_by_datetime(&conn, form.datetime_from, form.datetime_to)?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Deserialize, Serialize, Debug)]
struct PostWeightRequest {
    timestamp: i64,
    weight: f32,
    device_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct GetWeightRequest {
    datetime_from: DateTime<Local>,
    datetime_to: DateTime<Local>,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct FetchWeightResponse{
    weights: Vec<Weight>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Weight{
    id: i32,
    timestamp: DateTime<Local>,
    created_at: DateTime<Local>,
    weight: f32,
    device_id: i32,
}



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let pool = utils::db::establish_connection();
    let app_state = utils::state::AppState { pool: pool };
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(api)
            .app_data(Data::new(app_state.clone()))
    })
    .bind(("0.0.0.0", 8000))?
    .workers(1)
    .run()
    .await
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/weight")
            .route("", get().to(get_weight))
            .route("", post().to(post_weight)),
    )
    .service(web::scope("/").route("", get().to(hello)));
}
