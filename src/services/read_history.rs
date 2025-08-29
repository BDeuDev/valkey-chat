use actix_web::{get, HttpResponse, Responder};

use crate::storage::read_parquet_history::read_parquet_history;

#[get("/history")]
pub async fn read_history() -> impl Responder {
    match read_parquet_history("chat-export.parquet") {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}