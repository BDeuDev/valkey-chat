use crate::{
    controllers::types::ExportQuery,
    services::{
        export_service::ExportService, history_service::HistoryService,
        message_service::MessageService,
    },
    storage::parquet::read_from_bytes,
};
use actix_web::{HttpResponse, Responder, get, web};

#[get("/export")]
async fn export_messages_by_room(
    export_svc: web::Data<ExportService>,
    history_svc: web::Data<HistoryService>,
    msg_svc: web::Data<MessageService>,
    query: web::Query<ExportQuery>,
) -> impl Responder {
    let msgs_valkey = match msg_svc.get_all_messages(&query.room).await {
        Ok(m) => m,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error fetching messages: {e}"));
        }
    };

    let mut msgs_s3 = match history_svc.import_from_s3(&query.room).await {
        Ok(b) => read_from_bytes(b).unwrap(),
        Err(_) => Vec::new(),
    };

    let mut msgs = msgs_valkey;
    msgs.append(&mut msgs_s3);

    match export_svc.export_to_s3(msgs, &query.room).await {
        Ok(_) => HttpResponse::Ok().body("Exported and uploaded to S3!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
