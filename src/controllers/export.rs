use actix_web::{get, web, HttpResponse, Responder};
use crate::{services::{export::ExportService, message::MessageService}};

#[get("/export")]
async fn export_messages(export_svc: web::Data<ExportService>, msg_svc: web::Data<MessageService>) -> impl Responder {
    let msgs = match msg_svc.get_recent_messages("general").await {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching messages: {e}"));
        }
    };
    match export_svc.export_to_local(msgs).await {
        Ok(_) => HttpResponse::Ok().body("Exported and uploaded to S3!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
