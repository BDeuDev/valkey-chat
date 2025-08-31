use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;

#[get("/export")]
async fn export_messages(state: web::Data<AppState>) -> impl Responder {
    match state.export_service.export_to_local(&state).await {
        Ok(_) => HttpResponse::Ok().body("Exported and uploaded to S3!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
