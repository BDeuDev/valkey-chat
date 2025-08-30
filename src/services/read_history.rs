use actix_web::{get, HttpResponse, Responder};

use crate::storage::parquet::read;

#[get("/history")]
pub async fn read_history() -> impl Responder {
    match read("chat-export.parquet") {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}