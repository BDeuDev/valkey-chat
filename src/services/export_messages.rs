use actix_web::{get, web, HttpResponse, Responder};
use crate::storage::{messages::fetch, parquet::export};
use crate::AppState;

#[get("/export")]
async fn export_messages(state: web::Data<AppState>) -> impl Responder {
    let mut conn = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    let msgs = match fetch(&mut conn, "general").await {
        Ok(m) => m,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if let Err(e) = export(msgs, "chat-export.parquet") {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    HttpResponse::Ok().body("Mensajes exportados a chat-export.parquet")
}