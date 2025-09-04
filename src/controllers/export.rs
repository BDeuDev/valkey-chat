use actix_web::{get, web, HttpResponse, Responder};
use crate::{controllers::types::{ExportQuery,}, services::{export::ExportService, message::MessageService}};

#[get("/export")]
async fn export_messages_by_room(export_svc: web::Data<ExportService>, msg_svc: web::Data<MessageService>, query: web::Query<ExportQuery>) -> impl Responder {
    let msgs = match msg_svc.get_all_messages(&query.room).await {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error fetching messages: {e}"));
        }
    };
    match export_svc.export_to_s3(msgs, &query.room).await {
        Ok(_) => HttpResponse::Ok().body("Exported and uploaded to S3!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
