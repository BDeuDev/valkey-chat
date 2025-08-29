use actix_web::{get, web, HttpResponse, Responder};
use crate::storage::{fetch_messages::fetch_messages, export_to_parquet::export_to_parquet};
use crate::AppState;

#[get("/export")]
async fn export_messages(state: web::Data<AppState>) -> impl Responder {
    let mut conn = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let msgs = match fetch_messages(&mut conn, "general").await {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if let Err(e) = export_to_parquet(msgs, "chat-export.parquet") {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("Mensajes exportados a chat-export.parquet")
}