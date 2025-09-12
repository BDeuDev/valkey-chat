use actix_web::{HttpResponse, Responder, get, web};

use crate::{
    controllers::types::ImportQuery, services::history_service::HistoryService,
    storage::parquet::read_from_bytes,
};

#[get("/history")]
pub async fn get_history_by_room(
    history_svc: web::Data<HistoryService>,
    query: web::Query<ImportQuery>,
) -> impl Responder {
    let bytes = match history_svc.import_from_s3(&query.room).await {
        Ok(b) => b,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error importing s3 file: {e}"));
        }
    };

    match read_from_bytes(bytes) {
        Ok(msgs) => HttpResponse::Ok().json(msgs),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error reading bytes from s3 file: {e}"));
        }
    }
}
